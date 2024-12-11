#[proc_macro_attribute]
pub fn packet(attr : TokenStream, item : TokenStream) -> TokenStream {
    let input = item.clone();
    let input = parse_macro_input!(input as Item);
    match (&input) {
        Item::Struct(ItemStruct { ident, fields, .. }) => {


            let mut meta = Vec::new();
            for arg in attr.to_string().split(",").map(|arg| arg.split("=").map(|part| part.trim()).collect::<Vec<_>>()).filter(|arg| arg.iter().any(|part| !part.is_empty())) {
                if (arg.len() != 2) {
                    Span::call_site().error("Failed to parse attribute").emit();
                    return item;
                }
                match (arg[0]) {

                    "prefix" => {
                        let Ok(prefix) = parse_str::<Expr>(arg[1]) else {
                            Span::call_site().error("Given prefix value is not an expression").emit();
                            return item;
                        };
                        meta.push(quote!{ const PREFIX : usize = #prefix; });
                    },

                    "bound" => {
                        let Ok(bound) = parse_str::<Ident>(arg[1]) else {
                            Span::call_site().error("Given bound value is not an identifier").emit();
                            return item;
                        };
                        meta.push(quote!{ const BOUND : Bound = Bound::#bound; });
                    },

                    "stage" => {
                        let Ok(stage) = parse_str::<Ident>(arg[1]) else {
                            Span::call_site().error("Given stage value is not an identifier").emit();
                            return item;
                        };
                        meta.push(quote!{ const STAGE : Stage = Stage::#stage; });
                    },

                    _ => {
                        Span::call_site().error(format!("Unknown attribute key `{}`", arg[0])).emit();
                        return item;
                    }
                }
            }

            let (encode, decode) = match (fields) {
                Fields::Named(FieldsNamed { named, .. }) => {
                    let mut encode = Vec::new();
                    let mut decode = Vec::new();
                    for field @ Field { ident, .. } in named {
                        if let Some(ident) = ident {
                            encode.push(quote_spanned!{ field.span() => buf.encode_write(&self.#ident)?; });
                            decode.push(
                                quote_spanned!{ field.span() => #ident : buf.read_decode()?, },
                            );
                        } else {
                            let item2: TokenStream2 = item.into();
                            let error = quote_spanned!{ field.span() => compile_error!("Named fields without an identifier are not allowed in packet items"); };
                            return quote! {
                                #item2
                                #error
                            }
                            .into();
                        }
                    }
                    (quote! { #(#encode)* }, quote! { { #(#decode)* } })
                }
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                    let mut encode = Vec::new();
                    let mut decode = Vec::new();
                    for (i, field) in unnamed.iter().enumerate() {
                        let i = Index::from(i);
                        encode.push(quote_spanned!{ field.span() => buf.encode_write(&self.#i)?; });
                        decode.push(quote_spanned!{ field.span() => buf.read_decode()?, });
                    }
                    (quote! { #(#encode)* }, quote! { ( #(#decode)* ) })
                }
                Fields::Unit => (quote! {}, quote! {}),
            };
            let item2: TokenStream2 = item.into();
            (quote!{
                #[derive(Debug, Clone, PartialEq, Eq)]
                #item2
                impl PacketMeta for #ident { #(#meta)* }
                impl PacketEncode for #ident { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> { #encode Ok(()) } }
                impl PacketDecode for #ident { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> { Ok(Self #decode) } }
            }).into()


        },
        _ => {
            Span::call_site().error("Unsupported item").emit();
            item
        }
    }
}
