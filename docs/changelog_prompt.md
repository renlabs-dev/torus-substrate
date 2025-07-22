# Substrate Runtime Interface Changelog Prompt

First, start by reading the output for:
```bash
./scripts/diff-previous-tag.sh > temp/diff.diff
```

This automatically finds the correct previous runtime tag and generates the diff. Read `temp/diff.diff` repeatedly until you reach the end of the changes. Given the paths and line numbers, read the files on those offsets for _all_ Rust-related channges, create todos for each of the topics of change. Begin writing the change to `docs/changes/spec-<spec_version>.md`. First Generate a structured changelog of **all interface changes**:

1. **Extrinsics** (dispatchable calls)
2. **Events**
3. **Storage items** (on‑chain values affecting metadata and client reads)
4. **Structs & Enums** (types deriving `Encode` or `Decode`)
5. **Behavior changes** (new mechanisms or runtime‑level behavior)

For each change in categories 1–4, output a markdown list entry with:

- A **git diff hunk** snippet showing the exact additions and deletions.
- A **brief paragraph** combining conceptual change and motivation.

Follow these rules:

**A. Pallet discovery**

- Look under `runtime/pallets/` (or project root `pallets/`) and in `runtime/src/lib.rs` where pallets are `construct_runtime!`‑ed.

**B. Extrinsics**

- Diff each pallet’s `pub enum Call` or FRAME v2 `#[pallet::call]` items (additions/removals/arg‑type changes). This is VERY important. Pay attention to all extrinsics and functions annotated with `#[pallet::call_index(...)]`.

**C. Events**

- Diff `pub enum Event` or FRAME v2 `#[pallet::event]` variants.

**D. Storage**

- For FRAME v2: diff each `#[pallet::storage]` block.
- For FRAME v1: diff inside `decl_storage! {}` macros.

**E. Structs & Enums**

- Identify types with `#[derive(..., Encode, Decode, ...)]`.
- Only record new/deleted/renamed fields or type changes. Fields unchanged may be elided using `...`.

**F. Errors**

- **Exclude** new errors; do not list `#[pallet::error]` changes.

**G. Format specifics**

- Use **git diff style** for all snippets (`+`/`-`).
- Combine conceptual change and motivation in one paragraph following the snippet.
- Omit separate "Conceptual change" and "Motivation" headings.

**H. Behavior Changes**

- At the end of the output file, append a section titled `## Behavior Changes`.
- To extract behavior changes:
  1. For each changed extrinsic, locate its implementation function in the pallet's `#[pallet::call]` section (commonly in `src/lib.rs` or `src/call.rs`).
  2. Diff the function body to show key logic additions/removals affecting runtime behavior.
  3. Summarize how the code works, noting important branches, side effects, and state mutations.
- For each behavior change or new mechanism, add:

    ```markdown
    ### <BehaviorName>

    **What changed**: [Technical description of the code change]

    **Why it matters**: [Impact on users, validators, miners, or network economics]

    **Migration needed**: [Any action required from network participants]

    *Tests*: Reference specific test functions that validate this behavior.
    *Cross-pallet impact*: Note any effects on other pallets via API calls.
    ```

**I. Analysis Setup**

- **Test-driven behavior analysis**: Look for test changes in `tests/` directories and `_test.rs` files to understand new behaviors, modified logic, and real-world usage patterns. Check integration tests in `pallets/*/tests/` for cross-pallet dependencies and API trait usage.
- **Migration context**: Check `pallets/*/src/migrations.rs` files for storage schema changes, data transformations, and version increments indicating breaking changes.
- **Torus-specific patterns**: Focus on domain module changes (agent.rs, stake.rs, etc.), generic function patterns `pub fn name<T: Config>()`, API trait implementations, and new `saturating_*` arithmetic operations.

---

### Example Entry (escaped codeblock)

```markdown
- \```diff
  + pub enum Call<T: Config> {
  +     reserve_collateral { who: T::AccountId, amount: BalanceOf<T> },
  \```
  Introduces `reserve_collateral` to lock collateral separately from other operations, allowing finer‑grained control in multi‑step workflows. See [<BehaviorName>](#BehaviorName).

- \```diff
  + #[pallet::event]
  + pub enum Event<T> {
  +     CollateralReserved(T::AccountId, BalanceOf<T>),
  \```
  Emits an event when collateral is reserved, enabling front‑ends and indexers to track reservations in real time. See [<BehaviorName>](#BehaviorName).
```

Only add See <BehaviorName> if this change is related to a behavior change.
