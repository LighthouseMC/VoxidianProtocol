#![feature(proc_macro_diagnostic)]

use std::collections::HashMap;
use std::sync::Mutex;
use proc_macro::{Span, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::{
    Expr, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, Index, Item, ItemEnum, ItemStruct, LitBool, Type, Variant, Visibility,
    parse_macro_input, parse_str, spanned::Spanned,
};
use lazy_static::lazy_static;


include!("packet.rs");
include!("packet_part.rs");
include!("packet_full_decode.rs");
