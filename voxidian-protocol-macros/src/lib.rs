#![feature(
    proc_macro_diagnostic,
    proc_macro_span,
    let_chains
)]

use std::fs;
use proc_macro::{ Span, TokenStream };
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::{
    Expr, Field, Fields, FieldsNamed, FieldsUnnamed,
    Ident, Index, Item, ItemEnum, ItemStruct, LitBool,
    Meta, MetaList, Type, Variant, Visibility,
    parse_macro_input, parse_str,
    spanned::Spanned,
};


include!("packet.rs");
include!("packet_part.rs");
include!("packet_full_decode.rs");
