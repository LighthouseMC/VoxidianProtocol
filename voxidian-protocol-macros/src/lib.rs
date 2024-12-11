#![feature(proc_macro_diagnostic)]

use proc_macro::{Span, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::{
    Expr, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, Index, Item, ItemEnum, ItemStruct, Type, Variant,
    parse_macro_input, parse_str, spanned::Spanned,
};


include!("packet.rs");
include!("packet_enum.rs");
