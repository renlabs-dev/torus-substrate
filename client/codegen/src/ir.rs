use std::fmt::Debug;

use quote::ToTokens;
use syn::{Ident, Type};

#[derive(Debug, Clone, PartialEq)]
pub struct PalletPattern {
    pub name: Ident,
    pub storages: Vec<StoragePattern>,
    pub calls: Vec<CallPattern>,
}

/// Represents different types of Substrate storage patterns
#[derive(Debug, Clone, PartialEq)]
pub enum StoragePattern {
    /// Storage Value - no input parameters, single output value
    Value {
        name: String,
        pallet: String,
        return_type: String,
    },
    /// Storage Map - single key input, maps to single value
    Map {
        name: String,
        pallet: String,
        key_type: String,
        return_type: String,
    },
    /// Storage Double Map - two key inputs, maps to single value
    DoubleMap {
        name: String,
        pallet: String,
        key1_type: String,
        key2_type: String,
        return_type: String,
    },
    /// Storage N Map - multiple key inputs (N > 2), maps to single value
    NMap {
        name: String,
        pallet: String,
        key_types: Vec<String>,
        return_type: String,
    },
}

#[derive(Clone)]
pub struct CallPattern {
    pub name: Ident,
    pub params: Vec<(Ident, Type)>,
    pub ret: Type,
    pub pallet: Ident,
}

impl Debug for CallPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallPattern")
            .field("name", &self.name)
            .field(
                "params",
                &self
                    .params
                    .iter()
                    .map(|(ident, ty)| {
                        (
                            ident.to_token_stream().to_string(),
                            ty.to_token_stream().to_string(),
                        )
                    })
                    .collect::<Vec<(_, _)>>(),
            )
            .field("ret", &self.ret.to_token_stream().to_string())
            .field("pallet", &self.pallet)
            .finish()
    }
}

impl PartialEq for CallPattern {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.pallet == other.pallet
    }
}
