#[proc_macro_attribute]
pub fn packet_part(attr : TokenStream, item : TokenStream) -> TokenStream {
    let input = item.clone();
    let input = parse_macro_input!(input as Item);
    match (&input) {


        Item::Struct(item_struct @ ItemStruct { ident, fields, .. }) => {
            if (! attr.is_empty()) {
                Span::call_site().error("Attrs are not supported on struct `packet_part`s").emit();
                item
            } else {
                let (encode, decode) = match (fields) {

                    Fields::Named(FieldsNamed { named, .. }) => {
                        let mut encode = Vec::new();
                        let mut decode = Vec::new();
                        for field @ Field { ident, vis, .. } in named {
                            if let Some(ident) = ident {
                                if let Visibility::Public(_) = vis { } else {
                                    Span::call_site().warning(format!("Packet `{}` field `{}` is not public", item_struct.ident, ident)).emit();
                                }
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
                let item2 : TokenStream2 = item.into();
                quote!{
                    #[derive(Debug, Clone)]
                    #item2
                    impl PacketEncode for #ident { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> { #encode Ok(()) } }
                    impl PacketDecode for #ident { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> { Ok(Self #decode) } }
                }.into()
            }
        },


        Item::Enum(ItemEnum { ident, variants, .. }) => {


            let repr = parse_macro_input!(attr as Type);
            let discriminant_ident =
                parse_str::<Ident>(&format!("__{}Discriminant", ident)).unwrap();

            let mut encode = quote! {};
            let mut decode =
                quote! { let discriminant = Into::<i32>::into(buf.read_decode::<#repr>()?); };
            for Variant { ident, fields, .. } in variants {
                let (destructure, encode_variant, decode_variant) = match (fields) {
                    Fields::Named(FieldsNamed { named, .. }) => {
                        let mut destructure = Vec::new();
                        let mut encode = Vec::new();
                        let mut decode = Vec::new();
                        for field @ Field { ident, .. } in named {
                            if let Some(ident) = ident {
                                destructure.push(quote! { #ident, });
                                encode.push(
                                    quote_spanned! { field.span() => buf.encode_write(#ident)?; },
                                );
                                decode.push(
                                    quote_spanned! { field.span() => #ident : buf.read_decode()?, },
                                );
                            } else {
                                let item2: TokenStream2 = item.into();
                                let error = quote_spanned! { field.span() => compile_error!("Named fields without an identifier are not allowed in packet items"); };
                                return quote! {
                                    #item2
                                    #error
                                }
                                .into();
                            }
                        }
                        (
                            quote! { { #(#destructure),* } },
                            quote! { #(#encode)* },
                            quote! { { #(#decode)* } },
                        )
                    }
                    Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                        let mut destructure = Vec::new();
                        let mut encode = Vec::new();
                        let mut decode = Vec::new();
                        for (i, field) in unnamed.iter().enumerate() {
                            let ident = parse_str::<Ident>(&format!("x{}", i)).unwrap();
                            destructure.push(quote!{ #ident, });
                            encode.push(quote_spanned!{ field.span() => buf.encode_write(#ident)?; });
                            decode.push(quote_spanned!{ field.span() => buf.read_decode()?, });
                        }
                        (
                            quote!{ ( #(#destructure),* ) },
                            quote!{ #(#encode)* },
                            quote!{ ( #(#decode)* ) },
                        )
                    }
                    Fields::Unit => (quote! {}, quote! {}, quote! {}),
                };
                encode.extend(quote! { Self::#ident #destructure => {
                    buf.encode_write(#repr::from(#discriminant_ident::#ident as i32))?;
                    #encode_variant
                } });
                decode.extend(
                    quote! { if (#discriminant_ident::#ident as i32 == discriminant) {
                        return Ok(Self::#ident #decode_variant);
                    } },
                );
            }

            let item2: TokenStream2 = item.into();
            let mut discriminant_variants = quote! {};
            for variant in variants {
                let ident = &variant.ident;
                discriminant_variants.extend(quote! { #ident});
                if let Some((_, discriminant)) = &variant.discriminant {
                    discriminant_variants.extend(quote! { = #discriminant });
                }
                discriminant_variants.extend(quote! { , });
            }
            (quote!{
                #[repr(u32)]
                #[derive(Debug, Clone, PartialEq, Eq)]
                #item2
                #[repr(u32)]
                #[derive(Debug, Clone, PartialEq, Eq)]
                enum #discriminant_ident { #discriminant_variants }
                impl PacketEncode for #ident { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> { match (self) { #encode } Ok(()) } }
                impl PacketDecode for #ident { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> { #decode Err(DecodeError::InvalidData) } }
            }).into()


        },
        _ => {
            Span::call_site().error("Unsupported item").emit();
            item
        }
    }

}