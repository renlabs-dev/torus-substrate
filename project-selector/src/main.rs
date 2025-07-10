use std::{
    collections::{HashMap, HashSet},
    process::Command,
};

use anyhow::{Context, Result};
use clap::Parser;

use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::{
    Edition, Message, MetadataCommand, Node, NodeDep, Package, PackageId, Target, TargetKind,
};
use itertools::Itertools;
use project_model::{CargoConfig, CargoFeatures, Sysroot};
use toolchain::Tool;

#[derive(Parser)]
#[command(name = "project-selector")]
#[command(about = "Generate IDE-compatible project metadata from Cargo workspace")]
struct Args {
    /// Comma-separated list of crates to include
    selected: Option<String>,

    /// Comma-separated list of crates to block
    #[arg(long, short = 'b', default_value = "")]
    blocklist: String,

    /// Comma-separated list of crates to allow (overrides blocklist)
    #[arg(long, short = 'a', default_value = "")]
    allowlist: String,

    /// Maximum depth for dependency resolution
    #[arg(long, short = 'd', default_value = "3")]
    depth: usize,
}

/// Represents a resolved target within a package
struct ResolvedTarget<'a> {
    target: &'a Target,
    kind: &'a TargetKind,
    index: usize,
}

/// Contains all information about a crate including its package, targets, and dependencies
struct CrateInfo<'a> {
    package: &'a Package,
    targets: Vec<ResolvedTarget<'a>>,
    dependencies: Vec<(&'a PackageId, &'a NodeDep)>,
    build_output: Option<&'a BuildOutput>,
}

type NodeDesc<'a> = (&'a Node, Vec<(&'a PackageId, &'a NodeDep)>);

/// Contains the complete project context needed for crate resolution
struct ProjectContext<'a> {
    metadata: &'a cargo_metadata::Metadata,
    workspace_crates: HashSet<PackageId>,
    dependencies: HashMap<&'a PackageId, NodeDesc<'a>>,
    build_outputs: HashMap<PackageId, BuildOutput>,
    proc_macro_dylibs: Vec<(String, Utf8PathBuf)>,
    sysroot: project_model::Sysroot,
    matcher: Box<dyn Fn(&Package) -> bool>,
}

impl<'a> ProjectContext<'a> {
    fn load(metadata: &'a cargo_metadata::Metadata, args: &Args) -> Result<Self> {
        let cwd = metadata.workspace_root.as_path();
        let sysroot =
            project_model::Sysroot::discover(cwd.try_into().unwrap(), &Default::default());

        let cmd = build_command(
            &Default::default(),
            &Default::default(),
            &cwd.join("Cargo.toml"),
            cwd,
            &sysroot,
        )
        .context("Failed to build cargo command")?;

        let build_outputs = run_command(cmd).context("Failed to run cargo build command")?;

        let selected: Vec<_> = args
            .selected
            .as_deref()
            .unwrap_or_default()
            .split(',')
            .collect();
        let workspace_crates: HashSet<_> = metadata
            .packages
            .iter()
            .filter(|pkg| {
                pkg.manifest_path.starts_with(&metadata.workspace_root)
                    && (args.selected.as_ref().is_none_or(|s| s.is_empty())
                        || selected.contains(&pkg.name.as_ref()))
            })
            .map(|pkg| pkg.id.clone())
            .collect();

        let proc_macro_dylibs = discover_proc_macro_dylibs(&sysroot, cwd)?;

        let dependencies = get_indirect_dependencies(metadata, &workspace_crates, args.depth)?;

        let blocklist = args
            .blocklist
            .split(',')
            .filter(|s| !s.is_empty())
            .map(glob::Pattern::new)
            .collect::<Result<Vec<_>, _>>()?;
        let allowlist = args
            .allowlist
            .split(',')
            .filter(|s| !s.is_empty())
            .map(glob::Pattern::new)
            .collect::<Result<Vec<_>, _>>()?;

        let matcher = Box::new({
            let workspace_crates = workspace_crates.clone();

            move |pkg: &Package| {
                if workspace_crates.contains(&pkg.id) {
                    return true;
                }

                for pat in &allowlist {
                    if pat.matches(&pkg.name) {
                        return true;
                    }
                }

                for pat in &blocklist {
                    if pat.matches(&pkg.name) {
                        return false;
                    }
                }

                true
            }
        });

        Ok(ProjectContext {
            metadata,
            workspace_crates,
            dependencies,
            build_outputs,
            proc_macro_dylibs,
            sysroot,
            matcher,
        })
    }

    fn get_dependencies(&self, pkg_id: &PackageId) -> Vec<(&'a PackageId, &'a NodeDep)> {
        self.dependencies
            .get(pkg_id)
            .map(|(_, deps)| deps.clone())
            .unwrap_or_default()
    }

    fn find_package(&self, pkg_id: &PackageId) -> Option<&'a Package> {
        self.metadata.packages.iter().find(|pkg| &pkg.id == pkg_id)
    }
}

fn discover_proc_macro_dylibs(
    sysroot: &project_model::Sysroot,
    cwd: &Utf8Path,
) -> Result<Vec<(String, Utf8PathBuf)>> {
    let target_libdir = {
        let mut cargo_config = sysroot.tool(Tool::Cargo, cwd, &Default::default());
        cargo_config
            .args([
                "rustc",
                "-Z",
                "unstable-options",
                "--print",
                "target-libdir",
            ])
            .env("RUSTC_BOOTSTRAP", "1");

        if let Ok(it) = utf8_stdout(&mut cargo_config) {
            it
        } else {
            let mut cmd = sysroot.tool(Tool::Rustc, cwd, &Default::default());
            cmd.args(["--print", "target-libdir"]);
            utf8_stdout(&mut cmd).context("Failed to get target-libdir")?
        }
    };

    let proc_macro_dylibs = if let Some(target_libdir) = target_libdir {
        std::fs::read_dir(&target_libdir)
            .context("Failed to read target-libdir")?
            .filter_map(|entry| {
                let dir_entry = entry.ok()?;
                if dir_entry.file_type().ok()?.is_file() {
                    let path = dir_entry.path();
                    let extension = path.extension()?;
                    if extension == std::env::consts::DLL_EXTENSION {
                        let name = path.file_stem()?.to_str()?.split_once('-')?.0.to_owned();
                        let path = Utf8PathBuf::from_path_buf(path).ok()?;
                        eprintln!("{name} : {path}");
                        return Some((name, path));
                    }
                }
                None
            })
            .collect()
    } else {
        vec![]
    };

    Ok(proc_macro_dylibs)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let metadata = MetadataCommand::new()
        .exec()
        .context("Failed to load cargo metadata")?;

    let project_context = ProjectContext::load(&metadata, &args)?;

    let resolved_crates = resolve_crates(&project_context)?;

    let project_json = generate_project_json(&project_context, &resolved_crates)?;

    println!("{}", serde_json::to_string_pretty(&project_json)?);

    Ok(())
}

/// Resolve all crates and their information based on project context
fn resolve_crates<'a>(
    context: &'a ProjectContext<'a>,
) -> Result<HashMap<&'a PackageId, CrateInfo<'a>>> {
    let mut crate_infos = HashMap::new();

    for &pkg_id in context.dependencies.keys() {
        let Some(package) = context.find_package(pkg_id) else {
            continue;
        };

        if !(context.matcher)(package) {
            continue;
        }

        let mut targets = Vec::new();
        for (index, target) in package.targets.iter().enumerate() {
            for kind in &target.kind {
                targets.push(ResolvedTarget {
                    target,
                    kind,
                    index,
                });
            }
        }

        let crate_info = CrateInfo {
            package,
            targets,
            dependencies: context.get_dependencies(pkg_id),
            build_output: context.build_outputs.get(pkg_id),
        };

        crate_infos.insert(pkg_id, crate_info);
    }

    Ok(crate_infos)
}

/// Generate the final JSON project data
fn generate_project_json<'a>(
    context: &'a ProjectContext<'a>,
    resolved_crates: &HashMap<&'a PackageId, CrateInfo<'a>>,
) -> Result<json::ProjectJsonData> {
    let mut crates = Vec::new();
    let mut target_to_crate_idx: HashMap<(&PackageId, usize), usize> = HashMap::new();

    // first pass: create a crate entry for every target
    for (pkg_id, crate_info) in resolved_crates {
        for target in &crate_info.targets {
            let crate_idx = crates.len();
            target_to_crate_idx.insert((pkg_id, target.index), crate_idx);

            let edition = match crate_info.package.edition {
                Edition::E2015 => json::EditionData::Edition2015,
                Edition::E2018 => json::EditionData::Edition2018,
                Edition::E2021 => json::EditionData::Edition2021,
                _ => json::EditionData::Edition2024,
            };

            let mut env = HashMap::new();
            inject_cargo_package_env(&mut env, crate_info.package);
            inject_cargo_env(&mut env);
            inject_rustc_tool_env(&mut env, &crate_info.package.name, target.kind.clone());

            let mut cfg_list = Vec::new();
            if let Some(build_output) = crate_info.build_output {
                env.extend(build_output.envs.clone());
                cfg_list.extend(build_output.cfgs.clone());
            }

            if let Some((node, _)) = context.dependencies.get(pkg_id) {
                cfg_list.extend(node.features.iter().map(|f| format!(r#"feature="{f}""#)));
            }

            let proc_macro_dylib_path = if matches!(target.kind, TargetKind::ProcMacro) {
                crate_info
                    .build_output
                    .and_then(|bo| bo.proc_macro_dylib.clone())
                    .or_else(|| {
                        context
                            .proc_macro_dylibs
                            .iter()
                            .find(|(name, _)| {
                                name.trim_start_matches("lib") == crate_info.package.name.as_str()
                            })
                            .map(|(_, path)| path.clone())
                    })
            } else {
                None
            };

            let source = crate_info
                .build_output
                .and_then(|bo| bo.out_dir.as_ref())
                .map(|out_dir| json::CrateSource {
                    include_dirs: vec![out_dir.clone()],
                    exclude_dirs: Default::default(),
                });

            let crate_data = json::CrateData {
                display_name: Some(crate_info.package.name.to_string()),
                root_module: target.target.src_path.clone(),
                edition,
                version: Some(crate_info.package.version.to_string()),
                deps: vec![], // will be filled in second pass
                cfg_groups: Default::default(),
                cfg: json::CfgList(cfg_list),
                target: None,
                env,
                proc_macro_dylib_path,
                is_workspace_member: Some(context.workspace_crates.contains(pkg_id)),
                source,
                is_proc_macro: matches!(target.kind, TargetKind::ProcMacro),
                repository: crate_info.package.repository.clone(),
                build: Some(json::BuildData {
                    label: "build-info".into(),
                    build_file: target.target.src_path.clone(),
                    target_kind: target.kind.into(),
                }),
                proc_macro_cwd: None,
            };

            crates.push(crate_data);
        }
    }

    // second pass: fill in dependencies
    for (pkg_id, crate_info) in resolved_crates {
        for target in &crate_info.targets {
            let Some(&crate_idx) = target_to_crate_idx.get(&(pkg_id, target.index)) else {
                continue;
            };

            let mut deps = Vec::new();
            for (dep_pkg_id, dep) in &crate_info.dependencies {
                if let Some(dep_crate_info) = resolved_crates.get(dep_pkg_id) {
                    let dep_target = dep_crate_info
                        .targets
                        .iter()
                        .find(|t| matches!(t.kind, TargetKind::Lib))
                        .or_else(|| dep_crate_info.targets.first());

                    if let Some(dep_target) = dep_target {
                        if let Some(&dep_crate_idx) =
                            target_to_crate_idx.get(&(dep_pkg_id, dep_target.index))
                        {
                            deps.push(json::Dep {
                                krate: json::CrateArrayIdx(dep_crate_idx),
                                name: dep.name.to_string(),
                            });
                        }
                    }
                }
            }

            if matches!(
                target.kind,
                TargetKind::Example | TargetKind::Bench | TargetKind::Test
            ) {
                let pkg = &resolved_crates[pkg_id];
                if let Some(dep_crate_idx) = pkg
                    .targets
                    .iter()
                    .enumerate()
                    .find(|t| t.1.target.is_lib())
                    .and_then(|(idx, _)| target_to_crate_idx.get(&(pkg_id, idx)))
                {
                    deps.push(json::Dep {
                        krate: json::CrateArrayIdx(*dep_crate_idx),
                        name: pkg.package.name.replace('-', "_"),
                    });
                };
            }

            crates[crate_idx].deps = deps;
        }
    }

    let cargo_check = json::RunnableData {
        program: "cargo".to_string(),
        args: vec!["check".to_string(), "--message-format=json".to_string()],
        cwd: context.metadata.workspace_root.clone(),
        kind: json::RunnableKindData::Check,
    };

    Ok(json::ProjectJsonData {
        sysroot: context.sysroot.root().map(|p| p.to_string()),
        sysroot_src: context.sysroot.rust_lib_src_root().map(|p| p.to_string()),
        sysroot_project: None,
        crates,
        runnables: vec![cargo_check],
    })
}

fn get_indirect_dependencies<'a>(
    metadata: &'a cargo_metadata::Metadata,
    workspace_crates: &HashSet<PackageId>,
    max_depth: usize,
) -> Result<HashMap<&'a PackageId, NodeDesc<'a>>> {
    let mut result = HashMap::new();
    let resolve = metadata
        .resolve
        .as_ref()
        .context("No dependency resolution found in metadata")?;
    let dependency_map: HashMap<_, _> = resolve.nodes.iter().map(|node| (&node.id, node)).collect();

    let mut queue = std::collections::VecDeque::new();
    let mut visited = HashSet::new();

    for crate_id in workspace_crates {
        queue.push_back((crate_id, 0));
        visited.insert(crate_id);
    }

    while let Some((pkg_id, depth)) = queue.pop_front() {
        let Some(node) = dependency_map.get(pkg_id) else {
            continue;
        };

        let mut dependencies = Vec::with_capacity(node.deps.len());

        for dep in &node.deps {
            dependencies.push((&dep.pkg, dep));

            if depth < max_depth && !visited.contains(&dep.pkg) {
                queue.push_back((&dep.pkg, depth + 1));
                visited.insert(&dep.pkg);
            }
        }

        result.insert(&node.id, (*node, dependencies));
    }

    Ok(result)
}

fn build_command(
    config: &CargoConfig,
    allowed_features: &HashSet<String>,
    manifest_path: &Utf8Path,
    current_dir: &Utf8Path,
    sysroot: &Sysroot,
) -> std::io::Result<Command> {
    let mut cmd = sysroot.tool(Tool::Cargo, current_dir, &config.extra_env);

    cmd.args(["check", "--quiet", "--workspace", "--message-format=json"]);
    cmd.args(&config.extra_args);

    cmd.arg("--manifest-path");
    cmd.arg(manifest_path);

    if let Some(target_dir) = &config.target_dir {
        cmd.arg("--target-dir").arg(target_dir);
    }

    // --all-targets includes tests, benches and examples in addition to the
    // default lib and bins. This is an independent concept from the --target
    // flag below.
    if config.all_targets {
        cmd.arg("--all-targets");
    }

    if let Some(target) = &config.target {
        cmd.args(["--target", target]);
    }

    match &config.features {
        CargoFeatures::All => {
            cmd.arg("--all-features");
        }
        CargoFeatures::Selected {
            features,
            no_default_features,
        } => {
            if *no_default_features {
                cmd.arg("--no-default-features");
            }
            if !features.is_empty() {
                cmd.arg("--features");
                cmd.arg(
                    features
                        .iter()
                        .filter(|&feat| allowed_features.contains(feat))
                        .join(","),
                );
            }
        }
    }

    cmd.arg("--keep-going");

    Ok(cmd)
}

#[derive(Default)]
struct BuildOutput {
    cfgs: Vec<String>,
    envs: HashMap<String, String>,
    out_dir: Option<Utf8PathBuf>,
    proc_macro_dylib: Option<Utf8PathBuf>,
}

// TODO: wait for rust 1.90, https://github.com/rust-lang/cargo/pull/15674
fn run_command(mut cmd: Command) -> std::io::Result<HashMap<PackageId, BuildOutput>> {
    fn is_dylib(path: &Utf8Path) -> bool {
        match path.extension().map(|e| e.to_owned().to_lowercase()) {
            None => false,
            Some(ext) => matches!(ext.as_str(), "dll" | "dylib" | "so"),
        }
    }

    let out = cmd.stderr(std::process::Stdio::piped()).output()?;
    let out = String::from_utf8_lossy(&out.stdout);

    let mut outputs: HashMap<PackageId, BuildOutput> = HashMap::new();

    for line in out.lines() {
        let message: Message = serde_json::from_str(line).unwrap();

        match message {
            Message::BuildScriptExecuted(mut message) => {
                let output = outputs.entry(message.package_id).or_default();

                output.cfgs.extend(message.cfgs);
                output.envs.extend(message.env.drain(..));

                let out_dir = std::mem::take(&mut message.out_dir);
                if !out_dir.as_str().is_empty() {
                    output
                        .envs
                        .insert("OUT_DIR".to_string(), out_dir.to_string());
                    output.out_dir = Some(out_dir);
                }
            }
            Message::CompilerArtifact(message) => {
                if message
                    .target
                    .kind
                    .contains(&cargo_metadata::TargetKind::ProcMacro)
                {
                    if let Some(filename) = message.filenames.iter().find(|file| is_dylib(file)) {
                        let output = outputs.entry(message.package_id).or_default();

                        output.proc_macro_dylib = Some(filename.clone());
                    }
                }
            }
            Message::BuildFinished(_) => {}
            Message::TextLine(_) => {}
            _ => {}
        }
    }

    Ok(outputs)
}

/// Recreates the compile-time environment variables that Cargo sets.
///
/// Should be synced with
/// <https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates>
///
/// FIXME: ask Cargo to provide this data instead of re-deriving.
pub(crate) fn inject_cargo_package_env(env: &mut HashMap<String, String>, package: &Package) {
    // FIXME: Missing variables:
    // CARGO_BIN_NAME, CARGO_BIN_EXE_<name>

    let manifest_dir = package.manifest_path.parent().unwrap();
    let mut set = |k: &str, v: String| {
        env.insert(k.to_string(), v);
    };

    set("CARGO_MANIFEST_DIR", manifest_dir.to_string());
    set("CARGO_MANIFEST_PATH", package.manifest_path.to_string());

    set("CARGO_PKG_VERSION", package.version.to_string());
    set("CARGO_PKG_VERSION_MAJOR", package.version.major.to_string());
    set("CARGO_PKG_VERSION_MINOR", package.version.minor.to_string());
    set("CARGO_PKG_VERSION_PATCH", package.version.patch.to_string());
    set("CARGO_PKG_VERSION_PRE", package.version.pre.to_string());

    set("CARGO_PKG_AUTHORS", package.authors.join(":"));

    set("CARGO_PKG_NAME", package.name.to_string());
    set(
        "CARGO_PKG_DESCRIPTION",
        package.description.clone().unwrap_or_default(),
    );
    set(
        "CARGO_PKG_HOMEPAGE",
        package.homepage.clone().unwrap_or_default(),
    );
    set(
        "CARGO_PKG_REPOSITORY",
        package.repository.clone().unwrap_or_default(),
    );
    set(
        "CARGO_PKG_LICENSE",
        package.license.clone().unwrap_or_default(),
    );
    set(
        "CARGO_PKG_LICENSE_FILE",
        package
            .license_file
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default(),
    );
    set(
        "CARGO_PKG_README",
        package
            .readme
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default(),
    );

    set(
        "CARGO_PKG_RUST_VERSION",
        package
            .rust_version
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default(),
    );
}

pub(crate) fn inject_cargo_env(env: &mut HashMap<String, String>) {
    env.insert("CARGO".to_string(), Tool::Cargo.path().to_string());
}

pub(crate) fn inject_rustc_tool_env(
    env: &mut HashMap<String, String>,
    cargo_name: &str,
    kind: TargetKind,
) {
    _ = kind;
    // FIXME
    // if kind.is_executable() {
    //     env.set("CARGO_BIN_NAME", cargo_name);
    // }
    env.insert("CARGO_CRATE_NAME".to_string(), cargo_name.replace('-', "_"));
}

fn utf8_stdout(cmd: &mut Command) -> std::io::Result<Option<String>> {
    let output = cmd.output()?;
    if !output.status.success() {
        match String::from_utf8(output.stderr) {
            Ok(stderr) if !stderr.is_empty() => return Ok(None),

            _ => return Ok(None),
        }
    }

    if let Ok(s) = String::from_utf8(output.stdout) {
        Ok(Some(s.trim().to_owned()))
    } else {
        Ok(None)
    }
}

mod json {
    use std::collections::{HashMap, HashSet};

    use camino::Utf8PathBuf;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
    pub struct ProjectJsonData {
        pub sysroot: Option<String>,
        pub sysroot_src: Option<String>,
        pub sysroot_project: Option<Box<ProjectJsonData>>,
        #[serde(default)]
        // pub cfg_groups: HashMap<String, CfgList>,
        pub crates: Vec<CrateData>,
        #[serde(default)]
        pub runnables: Vec<RunnableData>,
    }

    #[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
    pub struct CrateData {
        pub display_name: Option<String>,
        pub root_module: Utf8PathBuf,
        pub edition: EditionData,
        #[serde(default)]
        pub version: Option<String>,
        pub deps: Vec<Dep>,
        #[serde(default)]
        pub cfg_groups: HashSet<String>,
        #[serde(default)]
        pub cfg: CfgList,
        pub target: Option<String>,
        #[serde(default)]
        pub env: HashMap<String, String>,
        pub proc_macro_dylib_path: Option<Utf8PathBuf>,
        pub is_workspace_member: Option<bool>,
        pub source: Option<CrateSource>,
        #[serde(default)]
        pub is_proc_macro: bool,
        #[serde(default)]
        pub repository: Option<String>,
        #[serde(default)]
        pub build: Option<BuildData>,
        #[serde(default)]
        pub proc_macro_cwd: Option<Utf8PathBuf>,
    }

    #[derive(Serialize, Deserialize, Default, Debug, Clone, Eq, PartialEq)]
    #[serde(rename = "edition")]
    pub enum EditionData {
        #[serde(rename = "2015")]
        Edition2015,
        #[serde(rename = "2018")]
        Edition2018,
        #[serde(rename = "2021")]
        Edition2021,
        #[default]
        #[serde(rename = "2024")]
        Edition2024,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
    pub struct BuildData {
        pub label: String,
        pub build_file: Utf8PathBuf,
        pub target_kind: TargetKindData,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct RunnableData {
        pub program: String,
        pub args: Vec<String>,
        pub cwd: Utf8PathBuf,
        pub kind: RunnableKindData,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub enum RunnableKindData {
        Check,
        Run,
        TestOne,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub enum TargetKindData {
        Bin,
        /// Any kind of Cargo lib crate-type (dylib, rlib, proc-macro, ...).
        Lib,
        Test,
    }

    impl From<&cargo_metadata::TargetKind> for TargetKindData {
        fn from(value: &cargo_metadata::TargetKind) -> Self {
            use cargo_metadata::TargetKind;

            match value {
                TargetKind::Bin => Self::Bin,
                TargetKind::Bench | TargetKind::Example | TargetKind::Test => Self::Test,
                TargetKind::ProcMacro
                | TargetKind::CustomBuild
                | TargetKind::CDyLib
                | TargetKind::DyLib
                | TargetKind::Lib
                | TargetKind::RLib
                | TargetKind::StaticLib
                | TargetKind::Unknown(_) => Self::Lib,
                _ => Self::Lib,
            }
        }
    }

    /// Identifies a crate by position in the crates array.
    ///
    /// This will differ from `Crate` when multiple `ProjectJson`
    /// workspaces are loaded.
    #[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Hash)]
    #[serde(transparent)]
    pub struct CrateArrayIdx(pub usize);

    #[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
    pub struct Dep {
        /// Identifies a crate by position in the crates array.
        #[serde(rename = "crate")]
        pub krate: CrateArrayIdx,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct CrateSource {
        pub include_dirs: Vec<Utf8PathBuf>,
        pub exclude_dirs: Vec<Utf8PathBuf>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default)]
    #[serde(transparent)]
    pub struct CfgList(pub Vec<String>);
}
