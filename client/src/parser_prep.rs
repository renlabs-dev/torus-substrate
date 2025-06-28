/*
Storages
Storage value - no input outputs value hold
Storage Map - one input key, standard dictionary
Storage Double Map - two input keys
Storage N Map - three input keys

What we need:
Wrappers for all of these.

How will they look like:
for storage value generate just `get_x` -> x
for storage map generate `get_x_by_y` and query_map -> returns whole map (iterates over keys)
for storage double map generate `get_x_by_y_by_z` and query_map -> returns whole map (iterates over keys)
for storage n map generate `get_x_by_y_by_z_by_...` and query_map -> returns whole map (iterates over keys)

Pseudo examples


Storage value:

/// The total amount of stake in the network.
#[pallet::storage]
pub type TotalStake<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

Our parser would process and output:

yields:
`get_total_stake -> u64 (balance of)`

Storage Map:

/// Known registered network agents indexed by the owner's key.
#[pallet::storage]
pub type Agents<T: Config> = StorageMap<_, Identity, AccountIdOf<T>, Agent<T>>;

Our parser would process and output:

yiels:
`get_agents(input: AccountId) -> Agent`
`query_map_agents -> dict[AccountId, Agnet]`


Storage Double Map:

// Map of staked tokens prefixed by the staked agent, and indexed by the staker
// keys mapping to the amount in tokens.
#[pallet::storage]
pub type StakedBy<T: Config> =
StorageDoubleMap<_, Identity, T::AccountId, Identity, T::AccountId, BalanceOf<T>>;

Our parser would process and output:

yields:
`get_staked_by_by_agent_by_staker(agent: AccountId, staker: AccountId) -> Balance`
`query_map_staked_by -> dict[AccountId, dict[AccountId, Balance]]`


Storage N Map

#[pallet::storage]
pub type AccumulatedStreamAmounts<T: Config> = StorageNMap<
    _,
    (
        NMapKey<Identity, T::AccountId>,
        NMapKey<Identity, StreamId>,
        NMapKey<Identity, PermissionId>,
    ),
    BalanceOf<T>,
>;

Our parser would process and output:

yields:
`get_accumulated_stream_amounts_by_account_by_stream_by_permission(account: AccountId, stream_id: StreamId, permission_id: PermissionId) -> Balance`
`query_map_accumulated_stream_amounts -> dict[AccountId, dict[StreamId, dict[PermissionId, Balance]]]`


## --- SUBXT Generated Metadata Structure ---


Below is a **zoom-in on exactly what `subxt 0.42.x` writes to `api.rs`** when you run

```bash
subxt codegen --url … | rustfmt > api.rs
```

### 0. Preamble & root wrapper

```rust
#![allow(clippy::all)]
#![allow(unused_imports, dead_code)]
pub mod api { /* everything else lives here */ }
```

* The outer attributes silence lint noise that would otherwise appear in such a huge generated file.
* The whole interface sits inside **`pub mod api { … }`** – that name is hard-coded in the code-generator (see `item_mod: parse_quote!(pub mod api {})` in the source). ([docs.rs][1])

---

### 1. Mini-index constants

Inside `api` the first things you meet are two static arrays built directly from `Metadata::pallets()` and `Metadata::runtime_api_traits()`:

```rust
pub static PALLETS: [&str; N] = [ /* "System", "Balances", … */ ];
pub static RUNTIME_APIS: [&str; M] = [ /* "BlockBuilder", … */ ];
```

They let you convert a pallet or runtime-API **name → numeric index** without touching the heavy metadata structures at runtime.

---

### 2. Top-level aliases & config glue

```rust
pub type DispatchError  = runtime_types::sp_runtime::DispatchError;
pub type RuntimeCall    = runtime_types::my_runtime::RuntimeCall;
pub type RuntimeEvent   = runtime_types::my_runtime::RuntimeEvent;

pub type ExtrinsicParams = subxt::config::SubstrateExtrinsicParams<Config>;

pub struct Config;
impl subxt::config::Config for Config {
    type Index        = u32;
    type BlockNumber  = u32;
    type Hash         = ::subxt::utils::H256;
    type AccountId    = ::subxt::utils::AccountId32;
    /* … other Substrate-specific associated types … */
}
```

`Config` connects the generated types to **Subxt’s generic client**; those associated types are copied from the runtime’s metadata (block hash type, account ID length, etc.).

---

### 3. One module per pallet

For every pallet *P* you get `pub mod p` (lower-snake-case):

```
balances/
├─ calls/        // XCM-style Call structs + TxPayload impls
├─ events/       // Event enums + StaticEvent impls
├─ storage/      // StorageAddress builders
├─ constants/    // ConstantAddress items
└─ errors.rs     // `pub type Error = …` (only if pallet defined an error)
```

#### 3.1 `calls/`

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transfer {
    pub dest: ::subxt::utils::MultiAddress,
    pub value: u128,
}
impl ::subxt::tx::Call for Transfer {
    const PALLET: &str   = "Balances";
    const FUNCTION: &str = "transfer";
    type Arguments = (Self::dest, Self::value);
}
```

Generated for **every extrinsic** (`PalletMetadata::calls()`).
Implements `subxt::tx::Call`, so you can do:

```rust
client
    .tx()
    .sign_and_submit_then_watch_default(&balances::calls::Transfer { … })
    .await?;
```

#### 3.2 `events/`

An enum per event variant plus a blanket `Event` impl so you can subscribe:

```rust
if let Event::Transfer(ev) = event.as_event::<balances::events::Transfer>()?
```

#### 3.3 `storage/`

Each item is a zero-sized *builder* that captures keys & hashers at the type level:

```rust
pub fn Account(
    who: ::subxt::utils::AccountId32
) -> ::subxt::storage::StorageAddress<
        pallet_balances::Account,  // return type
        subxt::config::PolkadotConfig, 
    > { … }
```

Multi-maps get a `pub struct AccountIter` with an `iter` helper, etc.

#### 3.4 `constants/`

```rust
pub const ExistentialDeposit: ::subxt::constants::ConstantAddress<
    u128, subxt::config::PolkadotConfig
> = ::subxt::constants::ConstantAddress::new("Balances", "ExistentialDeposit");
```

---

### 4. `runtime_types` – the flattened SCALE registry

The last (and largest) chunk is **`pub mod runtime_types { … }`**.
Every SCALE-encodable type that appears anywhere in the metadata ends up here, **namespaced to avoid clashes**:

```text
runtime_types
└── sp_runtime
    └── multiaddress
        └── enum MultiAddress { Id(AccountId32), Index(u32), … }

runtime_types
└── pallet_balances
    ├── pallet::Call       // original runtime call enum
    ├── pallet::Event
    └── types for storage structs, errors, etc.
```

Generation of this module is driven by the code-gen setting
`types_mod_ident: parse_quote!(runtime_types)` ([docs.rs][1])

Additional derives/attributes (`Encode`, `Decode`, `TypeInfo`, plus Subxt’s own `EncodeAsType`/`DecodeAsType`) are inserted automatically so the whole tree can be sent back over SCALE without you writing a single manual impl.


## Cheat-sheet recap

| Section            | What to grep for          | Source in metadata                   |
| ------------------ | ------------------------- | ------------------------------------ |
| Header & root      | `pub mod api {`           | hard-coded template ([docs.rs][1])   |
| Pallet list        | `static PALLETS`          | `Metadata::pallets()`                |
| Runtime APIs list  | `static RUNTIME_APIS`     | `Metadata::runtime_api_traits()`     |
| Top-level aliases  | `type DispatchError = …`  | `Metadata::dispatch_error_ty()` etc. |
| Pallet modules     | `pub mod balances {`      | `PalletMetadata`                     |
| Flattened registry | `pub mod runtime_types {` | SCALE type registry ([docs.rs][1])   |

--- X ---

*/
