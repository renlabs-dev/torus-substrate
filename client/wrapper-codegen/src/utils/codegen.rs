use crate::{
    InterfaceSource,
    ir::{PalletPattern, StoragePattern},
};
//use crate::utils::type_shortening::TypeShortener;
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, format_ident, quote};
use stringcase::pascal_case;
use syn::{Type, parse_str};
