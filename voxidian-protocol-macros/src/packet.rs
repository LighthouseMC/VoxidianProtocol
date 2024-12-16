#[proc_macro_attribute]
pub fn packet(attr : TokenStream, item : TokenStream) -> TokenStream {
    let input = item.clone();
    let input = parse_macro_input!(input as Item);
    match (&input) {
        Item::Struct(item_struct @ ItemStruct { attrs, vis, struct_token, ident, generics, fields, semi_token }) => {


            let MetaKVPairs { meta_prefix, meta_bound, meta_stage, meta_allow_priv } = match (MetaKVPairs::split(attr)) {
                Ok(meta) => meta,
                Err(e) => {
                    Span::call_site().error(e).emit();
                    return item
                }
            };
            let Some(meta_prefix ) = meta_prefix else { return quote_spanned!{ Span2::call_site() => compile_error!("No `prefix` value given" ); }.into() };
            let Some(meta_bound  ) = meta_bound  else { return quote_spanned!{ Span2::call_site() => compile_error!("No `bound` value given"  ); }.into() };
            let Some(meta_stage  ) = meta_stage  else { return quote_spanned!{ Span2::call_site() => compile_error!("No `stage` value given"  ); }.into() };
            let allow_priv = meta_allow_priv.is_some_and(|allow_priv| allow_priv);

            let struct_name = ident.to_string();
            if (! struct_name.ends_with(&format!("{}Packet", quote!{ #meta_bound }.to_string()))) {
                Span::call_site().warning(format!("Packet `{}` does not have standard suffix `{}Packet`", struct_name, meta_bound)).emit();
            }


            let (new_fields, encode, decode, debug) = match (fields) {

                Fields::Named(FieldsNamed { brace_token, named }) => {
                    let mut new_named = Punctuated::new();
                    let mut encode = Vec::new();
                    let mut decode = Vec::new();
                    let mut debug  = Vec::new();
                    for field @ Field { attrs, vis, mutability, ident, colon_token, ty } in named {
                        if let Some(ident) = ident {
                            if (! allow_priv) { if let Visibility::Public(_) = vis { } else {
                                Span::call_site().warning(format!("Packet `{}` field `{}` is not public", item_struct.ident, ident)).emit();
                            } }
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
                        } else {
                            return quote_spanned!{ field.span() => compile_error!("Named fields without an identifier are not allowed in packet items"); }.into();
                        }
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
                        if (! allow_priv) { if let Visibility::Public(_) = field.vis { } else {
                            Span::call_site().warning(format!("Packet `{}` field `{}` is not public", item_struct.ident, i)).emit();
                        } }
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
            (quote!{
                #(#attrs)* #vis #struct_token #ident #generics #new_fields #semi_token
                impl fmt::Debug for #ident { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result { #debug } }
                impl PacketMeta for #ident {
                    const PREFIX : usize = #meta_prefix;
                    const BOUND  : Bound = Bound::#meta_bound;
                    const STAGE  : Stage = Stage::#meta_stage;
                }
                impl PacketEncode for #ident { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> { #encode Ok(()) } }
                impl PacketDecode for #ident { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> { Ok(Self #decode) } }
            }).into()


        },
        _ => {
            quote_spanned!{ Span2::call_site() => compile_error!("Unsupported item"); }.into()
        }
    }
}


struct MetaKVPairs {
    meta_prefix     : Option<Expr>,
    meta_bound      : Option<Ident>,
    meta_stage      : Option<Ident>,
    meta_allow_priv : Option<bool>
}
impl MetaKVPairs {
    fn split(input : TokenStream) -> Result<Self, String> {
        let mut out = Self {
            meta_prefix     : None,
            meta_bound      : None,
            meta_stage      : None,
            meta_allow_priv : None
        };
        for arg in input.to_string().split(",").map(|arg| arg.split("=").map(|part| part.trim()).collect::<Vec<_>>()).filter(|arg| arg.iter().any(|part| !part.is_empty())) {
            if (arg.len() != 2) {
                return Err("Failed to parse attribute".to_string());
            }
            match (arg[0]) {

                "prefix" => {
                    let Ok(prefix) = parse_str::<Expr>(arg[1]) else {
                        return Err("Given `prefix` value is not an expression".to_string());
                    };
                    if let Some(_) = out.meta_prefix {
                        return Err("`prefix` specified multiple times".to_string());
                    }
                    out.meta_prefix = Some(prefix);
                },

                "bound" => {
                    let Ok(bound) = parse_str::<Ident>(arg[1]) else {
                        return Err("Given `bound` value is not an identifier".to_string());
                    };
                    if let Some(_) = out.meta_bound {
                        return Err("`bound` specified multiple times".to_string());
                    }
                    out.meta_bound = Some(bound);
                },

                "stage" => {
                    let Ok(stage) = parse_str::<Ident>(arg[1]) else {
                        return Err("Given `stage` value is not an identifier".to_string());
                    };
                    if let Some(_) = out.meta_stage {
                        return Err("`stage` specified multiple times".to_string());
                    }
                    out.meta_stage = Some(stage);
                },

                "allow_priv" => {
                    let Ok(allow_priv) = parse_str::<LitBool>(arg[1]) else {
                        return Err("Given `allow_priv` value is not a boolean".to_string());
                    };
                    if let Some(_) = out.meta_allow_priv {
                        return Err("`allow_priv` specified multiple times".to_string());
                    }
                    let allow_priv = match (quote!{ #allow_priv }.to_string().as_str()) {
                        "true"  => true,
                        "false" => false,
                        _ => { return Err("`allow_priv` is not a boolean".to_string()) }
                    };
                    out.meta_allow_priv = Some(allow_priv);
                }

                _ => {
                    return Err(format!("Unknown attribute key `{}`", arg[0]));
                }
            }
        }
        Ok(out)
    }
}
