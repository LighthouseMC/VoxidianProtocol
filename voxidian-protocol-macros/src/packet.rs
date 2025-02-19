use proc_macro::TokenStream;
use inflector::Inflector;
use proc_macro2::Ident;
use quote::{ quote, quote_spanned };
use syn::{ parse_macro_input, parse_str, Field, Fields, FieldsNamed, FieldsUnnamed, Index, Item, ItemStruct, LitStr, Meta };
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use crate::{PacketData, PACKETS_DATA};

pub(crate) fn packet_impl(attr : TokenStream, item : TokenStream) -> TokenStream {
    let packet_id    = parse_macro_input!( attr as LitStr ).value();
    let packet_id_tc = packet_id.split("/").last().unwrap().to_title_case().replace(" ", "");
    let PacketData { prefix, bound, stage } = {
        let Some(packet_data) = PACKETS_DATA.packets.get(&packet_id) else { panic!("Unknown packet {:?}", packet_id) };
        *packet_data
    };

    let input = item.clone();
    let input = parse_macro_input!(input as Item);
    match (&input) {
        Item::Struct(ItemStruct { attrs, vis, struct_token, ident, generics, fields, semi_token }) => {

            let struct_name = ident.to_string();
            let expected_name = format!("{}{:?}{:?}Packet", packet_id_tc, bound, stage);
            if (struct_name != expected_name) {
                panic!("Packet {:?} does not have standard name {:?}", struct_name, expected_name);
            }


            let (new_fields, encode, decode, debug) = match (fields) {

                Fields::Named(FieldsNamed { brace_token, named }) => {
                    let mut new_named = Punctuated::new();
                    let mut encode = Vec::new();
                    let mut decode = Vec::new();
                    let mut debug  = Vec::new();
                    for field @ Field { attrs, vis, mutability, ident, colon_token, ty } in named {
                        if let Some(ident) = ident {
                            let ident_str = ident.to_string();
                            let mut new_attrs = Vec::new();
                            let mut redacted  = false;
                            for attr in attrs { if let Meta::Path(path) = &attr.meta && let Some(ident) = path.get_ident() && (ident.to_string() == "redacted") {
                                redacted = true;
                            } else {
                                new_attrs.push(attr.clone());
                            } }
                            new_named.push(Field { attrs : new_attrs, vis : vis.clone(), mutability : mutability.clone(), ident : Some(ident.clone()), colon_token : *colon_token, ty : ty.clone() });
                            encode.push(quote_spanned!{ field.span() => buf.encode_write(&self.#ident)?; });
                            decode.push(quote_spanned!{ field.span() => #ident : buf.read_decode()?, },);
                            debug.push(if (redacted) {
                                quote_spanned!{ field.span() => .field_with(#ident_str, |f| write!(f, "<REDACTED>")) }
                            } else {
                                quote_spanned!{ field.span() => .field(#ident_str, &self.#ident) }
                            })
                        } else { unreachable!() }
                    }
                    (
                        Fields::Named(FieldsNamed { brace_token : *brace_token, named : new_named }),
                        quote!{ #(#encode)* },
                        quote!{ { #(#decode)* } },
                        quote!{ f.debug_struct(#struct_name) #(#debug)* .finish() }
                    )
                }

                Fields::Unnamed(FieldsUnnamed { paren_token, unnamed }) => {
                    let mut new_unnamed = Punctuated::new();
                    let mut encode = Vec::new();
                    let mut decode = Vec::new();
                    let mut debug  = Vec::new();
                    for (i, field @ Field { attrs, vis, mutability, ident : _, colon_token, ty }) in unnamed.iter().enumerate() {
                        let i = Index::from(i);
                        let mut new_attrs = Vec::new();
                        let mut redacted  = false;
                        for attr in attrs { if let Meta::Path(path) = &attr.meta && let Some(ident) = path.get_ident() && (ident.to_string() == "redacted") {
                            redacted = true;
                        } else {
                            new_attrs.push(attr.clone());
                        } }
                        new_unnamed.push(Field { attrs : new_attrs, vis : vis.clone(), mutability : mutability.clone(), ident : None, colon_token : *colon_token, ty : ty.clone() });
                        encode.push(quote_spanned!{ field.span() => buf.encode_write(&self.#i)?; });
                        decode.push(quote_spanned!{ field.span() => buf.read_decode()?, });
                        debug.push(if (redacted) {
                            quote_spanned!{ field.span() => .field_with(|f| write!(f, "<REDACTED>")) }
                        } else {
                            quote_spanned!{ field.span() => .field(&self.#i) }
                        })
                    }
                    (
                        Fields::Unnamed(FieldsUnnamed { paren_token : *paren_token, unnamed : new_unnamed }),
                        quote!{ #(#encode)* },
                        quote!{ ( #(#decode)* ) },
                        quote!{ f.debug_tuple(#struct_name) #(#debug)* .finish() }
                    )
                }

                Fields::Unit => (
                    Fields::Unit,
                    quote!{},
                    quote!{},
                    quote!{ write!(f, #struct_name) }
                ),

            };
            let bound_ident = parse_str::<Ident>(&format!("{:?}", bound)).unwrap();
            let stage_ident = parse_str::<Ident>(&format!("{:?}", stage)).unwrap();
            (quote!{
                #[derive(Clone)]
                #(#attrs)* #vis #struct_token #ident #generics #new_fields #semi_token
                impl fmt::Debug for #ident { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result { #debug } }
                impl PacketMeta for #ident {
                    const PREFIX : u8    = #prefix;
                    const BOUND  : Bound = Bound::#bound_ident;
                    const STAGE  : Stage = Stage::#stage_ident;
                }
                impl PacketEncode for #ident { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> { #encode Ok(()) } }
                impl PacketDecode for #ident { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> { Ok(Self #decode) } }
            }).into()


        },
        _ => { panic!("Unsupported item") }
    }
}
