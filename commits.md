commit 498642de53cda8d8b98340a0616eb1244089f680
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: GitHub <noreply@github.com>

    chore: final migrations tweaks (#118)

    We are getting ready for the mainnet release on July 7th. This change
    finishes the name porting logic converting agent names to complying
    namespace names, by lowering and removing whitespace from the names.

commit 5d79e22fd8d36e56538adcc303cca575ebc4d3b5
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: GitHub <noreply@github.com>

    fix(namespaces): prevent de-registering root (#117)

    This patch prevents a registered agent from de-registering the agent
    entry (`agent.<name>`) through the `delete_namespace` extrinsic.

commit e75b3506261643ff2f5ac219ebd85aa6bfed6052
Merge: b9f14a6 954a248
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: Luiz Carvalho <luizcmpc@gmail.com>

    Merge remote-tracking branch 'main' into dev

commit b9f14a6bf2b953891f1c3c73e2836fdd04a4e07a
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: Luiz Carvalho <luizcmpc@gmail.com>

    chore(permission0): accumulate distribution remainder

commit 954a2485badc49c1ed35e8015b396fb8bad8e4c7
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: GitHub <noreply@github.com>

    fix(torus0/stake): refund stake before clearing map (#113)

    This patch fixes the de-registration code to clear the map after the
    stake has been refunded to the accounts. A previous test case ensure the
    _wrong_ behavior, which is now fixed.

    An agent this week deregistered and got its tokens locked because of
    order of operations. Luckly, because the TotalStake storage is still
    wrong, the imbalance represents the amount needed to be refunded without
    minting new tokens.

    Closes CHAIN-103.


    <!-- This is an auto-generated comment: release notes by coderabbit.ai
    -->
    ## Summary by CodeRabbit

    - **Bug Fixes**
    - Resolved an issue in staking logic to ensure proper refunding of stake
    during agent unregistration and runtime upgrades.
    - Updated migration logic to correct stake imbalances from previous
    versions.

    - **Refactor**
      - Improved clarity and maintainability of stake removal logic.
      - Enhanced parameter naming for better readability.
      - Modified operation order in agent unregistration for consistency.

    - **Tests**
    - Revised and renamed tests to better reflect updated agent
    unregistration and staking behaviors.

    - **Chores**
    - Updated runtime and storage version numbers to reflect the latest
    changes.
    - Introduced workspace-wide linting configurations for consistent code
    quality.
      - Enforced stricter Clippy lint rules across the workspace.
    - Applied saturating arithmetic in various runtime and pallet modules to
    prevent overflow issues.
    - Improved arithmetic safety and robustness in emission and governance
    modules.
    - Refined author identification and gas limit calculations with safer
    arithmetic operations.
    <!-- end of auto-generated comment: release notes by coderabbit.ai -->

commit d881aab6b97af3e9d71255aa15b90a4b5ac9aaec
Author: João Victor <65056371+devwckd@users.noreply.github.com>
Commit: GitHub <noreply@github.com>

    docs: document faucet usage and proof-of-work (#112)

    Closes CHAIN-90

commit fe22d9b9cff70c8d6b2a63d8a7bc15efb3a263c7
Author: João Victor <65056371+devwckd@users.noreply.github.com>
Commit: Luiz Carvalho <luizcmpc@gmail.com>

    feat: transfer fees to treasury (#78)

    Closes CHAIN-60

commit beb3529824587ecca54380e9730d6ab9ed3b0f63
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: GitHub <noreply@github.com>

    refac(torus0/namespace): require agent prefix (#111)

commit a9dba2b8a04ca1a19c11fcaaccaa6ee73c57233d
Author: DaviPtrs <davispetris@gmail.com>
Commit: DaviPtrs <davispetris@gmail.com>

    use Ubicloud

commit 10b4a0f9b7131d66edfbae13fecb88e7522f3d22
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: GitHub <noreply@github.com>

    chore: update burn parameters (#110)

    This change updates the burn parameters as well as namespace fees.

    Closes CHAIN-101.

commit a94678e562213b8d84977bee46dcf5be61b3139f
Author: João Victor <65056371+devwckd@users.noreply.github.com>
Commit: GitHub <noreply@github.com>

    docs: add testnet deploy steps doc (#94)

    Adds a doc containing the steps needed to deploy the runtime.

commit 6de5cb29be3395dcbf553e276b4278f2045b698e
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: GitHub <noreply@github.com>

    fix(permission0): allow delegator to delegate emission to itself (#107)

    After discussing how the emissions will be initially used, it was clear
    to us that there are no cases where it makes sense to delegate emissions
    permission to another agent. In this case, the best way to delegate is
    to itself, which allows the agent to control how the emissions will flow
    and edit the targets at will.

commit 04288e82ad5692430a50dc01fe215d0a19aa978d
Author: João Victor <65056371+devwckd@users.noreply.github.com>
Commit: GitHub <noreply@github.com>

    feat: add curator agent freezing toggling (#109)

    Closes CHAIN-100

commit e94b5b1205844e6ec9deb8c20f441850b9920684
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: GitHub <noreply@github.com>

    feat(namespace): cost calculation rpc (#106)

    Adds a new RPC call, `torus0_namespacePathCreationCost`, that calculates
    the total fee and deposit needed to create a new namespace entry.

commit 240d29b3067954c0ac9381c26edcc9cb77ea4a34
Author: João Victor <65056371+devwckd@users.noreply.github.com>
Commit: GitHub <noreply@github.com>

    feat: add stream emission permission editing (#104)

    Closes CHAIN-95

    ---------

    Co-authored-by: Luiz Carvalho <luizcmpc@gmail.com>

commit 78a648160a1980c34a42e14ad23c004c3604e5fc
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: GitHub <noreply@github.com>

    feat(namespaces): initial impl (#105)

    This is the initial implementation for namespaces. Please look into
    https://github.com/renlabs-dev/torus-substrate/blob/feat/namespaces-initial-impl/docs/namespace.md.

    Closes CHAIN-97.

commit 87cb3d338c84c10328e4f2f7ca324bbb35edfb89
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: GitHub <noreply@github.com>

    refac(permission0): general pre-release tweaks (#103)

    This change serves more as a QoL update then everything.

    The previous ID generation could allow for duplicate IDs in very
    specific cases, and that is now handled. When the enforcement
    authorities change, all ongoing enforcement votes for that permission
    are wiped. We cover more cases of input validation.

commit a92625615431ccfc737b6d6101e575021dcf8547
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: Luiz Carvalho <luizcmpc@gmail.com>

    chore: bump spec version to 15

commit e012a9945cc49a0738c03db4ffefb6e7de729966
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: Luiz Carvalho <luizcmpc@gmail.com>

    fix(testnet): add code substitute

    test(emission0): calculate rational correctly with rounding

commit c6e17029acbd15e942a3942356a92dfa3d1a3c38
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: Luiz Carvalho <luizcmpc@gmail.com>

    fix: ban arithmetic side effects

commit 1f22d1b76d91bd436aad8ae01c1f95912c3bd871
Author: João Victor <65056371+devwckd@users.noreply.github.com>
Commit: GitHub <noreply@github.com>

    feat: increase faucet tokens (#102)

commit 8e69b45583d05a4058226493888c3d3cce3e5dbe
Author: João Victor <65056371+devwckd@users.noreply.github.com>
Commit: GitHub <noreply@github.com>

    feat: enable testnet features on localnode (#101)

commit b73f239a1fa7903b1e244fa920c2e5a66d516a51
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: GitHub <noreply@github.com>

    fix(permission0): until block duration is inclusive

commit fc6cbdff9990aa0941deebba82ec628c2ea96e54
Author: Kelvin Steiner <me@steinerkelvin.dev>
Commit: Kelvin Steiner <me@steinerkelvin.dev>

    docs: improve documentation and add Claude Code guidance

    - Add CLAUDE.md with repository guidance for Claude Code assistant
    - Standardize shell code blocks to use 'sh' instead of 'bash'
    - Add spell checker words to VS Code settings
    - Simplify README project description for clarity
    - Add temporary files pattern to .gitignore

    🤖 Generated with [Claude Code](https://claude.ai/code)

    Co-Authored-By: Claude <noreply@anthropic.com>

commit b421a44c19b37eaa57277fb94a00418864fe6567
Author: João Victor <65056371+devwckd@users.noreply.github.com>
Commit: GitHub <noreply@github.com>

    chore: bump spec (#99)

commit 491524fdc87cd33477be8c713d2f48c4e0508e32
Author: Davi Petris <37764531+DaviPtrs@users.noreply.github.com>
Commit: GitHub <noreply@github.com>

    Fix the code coverage bucket URL (#98)

    # Pull Request Checklist

    Before submitting this PR, please make sure:

    - [ ] You have run `cargo clippy` and addressed any warnings
    - [ ] You have added appropriate tests (if applicable)
    - [ ] You have updated the documentation (if applicable)
    - [ ] You have reviewed your own code
    - [ ] You have updated changelog (if applicable)

    ## Description

    Please provide a brief description of the changes in this PR.

    ## Related Issues

    Please link any related issues here

commit d8b2393f7d6e7208fb49e4c0250b708c8a36ab64
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: GitHub <noreply@github.com>

    feat(permission0): create new curator scope (#97)

    Creates a new curator scope for the permission0 system. This change
    supersedes the `Curators` storage value in the governance pallet.

    A new extrinsic was added to `permission0`:
    ```rust
    pub fn delegate_curator_permission(
            origin: OriginFor<T>,
            recipient: T::AccountId,
            flags: u32,
            cooldown: Option<BlockNumberFor<T>>,
            duration: PermissionDuration<T>,
            revocation: RevocationTerms<T>,
    ) ;
    ```

    For now, only the sudo (root) key is allowed to delegate new permissions,
    which we will do through a multisig dispatch. Future versions will allow
    curators to re-delegate permissions. The new extrinsic allows setting
    different sub-permissions for curator actions:

    ```rust
            /// Permission to review and process agent applications
            const APPLICATION_REVIEW = 0b0000_0010;
            /// Permission to manage the whitelist (add/remove accounts)
            const WHITELIST_MANAGE   = 0b0000_0100;
            /// Permission to apply penalty factors to agents
            const PENALTY_CONTROL    = 0b0000_1000;
    ```

    The value for `0b1` is reserved for future use and internally assigned
    as `ROOT`.

    The `flags` field is a bitmask, so OR the different flags together to
    get your very own personalized permission.

    Finally, I removed the old extrinsics for curators in the governance
    pallet.

commit 000fe75de4da4a6c7719e55ee3ccc497a649fdb3
Author: João Victor <65056371+devwckd@users.noreply.github.com>
Commit: GitHub <noreply@github.com>

    feat: add faucet pallet (#96)

    This PR adds the faucet pallet.

    Closes CHAIN-85

commit 759e79c9932a4027ba9fe797547769db258869d0
Author: João Victor <65056371+devwckd@users.noreply.github.com>
Commit: GitHub <noreply@github.com>

    chore: publish autogenerated docs to github pages on push to the `main` branch (#93)

    Adds a CI task that generates the pallet docs and uploads them to
    [github
    pages](http://renlabs-dev.github.io/torus-substrate/pallet_torus0)
    Closes CHAIN-75

commit 44f90b8bcac29d41705d8a915a266d1ba021ff46
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: Luiz Carvalho <luizcmpc@gmail.com>

    feat(permission0): implement multi-sig controllers

commit 3309fa6d81c54318688db13e070be9262ba7799a
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: Luiz Carvalho <luizcmpc@gmail.com>

    feat(permission0): implement revoking by arbiters

commit e24e7e6c3e6ba75b680d38fb5fa106893da4d632
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: Luiz Carvalho <luizcmpc@gmail.com>

    feat(permission0): initial implementation (#87)

    This is the first version for the permission0 pallet. Effectively
    implements emission recursion.

    ---------

    Co-authored-by: devwckd <dev.wckd@gmail.com>

commit 88367b86cab586c5bc3651fc005f8db35bb78558
Merge: 06ec269 a546134
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: Luiz Carvalho <luizcmpc@gmail.com>

    Merge "feat: transfer fees to treasury (#78)" into dev

commit 06ec2691fbed5cb7a604929fe474e8a961d0098d
Author: João Victor <65056371+devwckd@users.noreply.github.com>
Commit: GitHub <noreply@github.com>

    feat: add cooldown and event to agent update extrinsic (#88)

    Closes CHAIN-66

    ---------

    Co-authored-by: Luiz Carvalho <luizcmpc@gmail.com>

commit df671d1ae694b960358c426f36c21424c39102bd
Author: Kelvin Steiner <me@steinerkelvin.dev>
Commit: Kelvin Steiner <me@steinerkelvin.dev>

    chore: add devcontainer.json; organizer Github Action files

commit 4456ef882f43d0bdf4597b1d7f84d2d5144938ba
Author: João Victor <65056371+devwckd@users.noreply.github.com>
Commit: GitHub <noreply@github.com>

    feat: make xtask spec generation tasks go to stdout when no output is… (#89)

    Closes CHAIN-72

commit 27aab25ad4ef94fed31b4890a82aa826c2b68786
Author: João Victor <65056371+devwckd@users.noreply.github.com>
Commit: GitHub <noreply@github.com>

    feat: build and push xtask cli tool to github packages (#86)

    This PR adds the `Docker build and push xtask cli tool` action.

    When code that changes anything related to the xtask is pushed to the
    branch `dev` or `main` this action builds a docker image and publishes
    it on github packages.

    The image tag is `ghcr.io/renlabs-dev/torus-xtask:{commit small sha}`
    with the addition of `ghcr.io/renlabs-dev/torus-xtask:latest` if it's
    pushed to `main`.

    Closes CHAIN-71

commit 2532ff757de596113ec0d7ce22946f773771b706
Author: Kelvin Steiner <me@steinerkelvin.dev>
Commit: Kelvin Steiner <me@steinerkelvin.dev>

    chore: disable `cargo-llvm-cov` on flake shell (marked as broken); unify nixpkgs inputs (follows); bump flake

commit 375bdaad1c4b937ca0acf491b01929161ae3e5fe
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: GitHub <noreply@github.com>

    chore: new readme (#85)

    Updates the README and creates a new CONTRIBUTING.md.

commit c00bab0cbacd800a2fe6e4e6d86bf93a77dbf96d
Author: João Victor <65056371+devwckd@users.noreply.github.com>
Commit: GitHub <noreply@github.com>

    test: improve `torus0` and `governance` pallet tests  (#84)

    Closes CHAIN-70

commit a5461345f4a318f58465a922f69e43a285a4191e
Author: João Victor <65056371+devwckd@users.noreply.github.com>
Commit: GitHub <noreply@github.com>

    feat: transfer fees to treasury (#78)

    Closes CHAIN-60

commit 95ff1c3bd962a023e66577c620b0ee24bc073d80
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: GitHub <noreply@github.com>

    chore: improve proposal and pruning coverage (#83)

    Improves coverage for some governance files.

commit ff40ef2a2afc075b99e611c55717e525663858cb
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: GitHub <noreply@github.com>

    chore: drop substrate-fixed types (#82)

    This is a large one. We've been facing issues with the precision
    provided by `substrate-fixed` types for a while now.

    This commit drops this dependency in favor of the SDK's provided `Fixed`
    types, which handle 18 decimals of precision correctly for most cases.
    The change required various tweaks to how we test things. I also
    expanded coverage for the emission code.

commit 15ebf95377f9bd2caac7de637e0146bbe53485e3
Author: João Victor <65056371+devwckd@users.noreply.github.com>
Commit: GitHub <noreply@github.com>

    feat: add code coverage tooling & reports (#73)

    Closes CHAIN-46

commit 0e7b0c7dfaca77c86ac9105e833d6669bf9f5e67
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: GitHub <noreply@github.com>

    fix: automatically delegate weights to allocator (#81)

    Updates weight control code to better handle the current state. This
    means all agents will automatically delegate weights to the allocator.
    Closes CHAIN-69.

commit 151076ab56545135457c1cad0271f1749562c21a
Author: Luiz Carvalho <luizcmpc@gmail.com>
Commit: GitHub <noreply@github.com>

    chore: remove old commune stuff (#80)

    Removes unused storage values from Commune times.

    Storage values removed:
    * `MinAllowedWeights`,
    * `MaxAllowedWeights`,
    * `MinStakePerWeight`,
    * `RegistrationBlock` (in favor of `registration_block` within the Agent
    value).

    Fields removed from `GlobalParamsData`:
    * `max_allowed_weights`,
    * `min_stake_per_weight`.

commit 2091d692f93f44afd917b2ac8158a15daadbdca5
Author: gabrielle oliveira <gabrielle1guim@gmail.com>
Commit: GitHub <noreply@github.com>

    docs: improve torus substrate docstrings (#71)

    Adds docstrings to most storage values and core functions. Closes
    CHAIN-45.

    ---------

    Co-authored-by: Luiz Carvalho <luizcmpc@gmail.com>
