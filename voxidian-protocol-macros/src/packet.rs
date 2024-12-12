type Stage = String;
type Bound = String;
#[derive(Debug)]
struct Packet {
    prefix      : String,
    struct_name : String
}

lazy_static!{
    static ref PACKET_REFLECTION : Mutex<HashMap<Stage, HashMap<Bound, Vec<Packet>>>> = Mutex::new(HashMap::new());
}


#[proc_macro_attribute]
pub fn packet(attr : TokenStream, item : TokenStream) -> TokenStream {
    let input = item.clone();
    let input = parse_macro_input!(input as Item);
    match (&input) {
        Item::Struct(item_struct @ ItemStruct { ident, fields, .. }) => {


            let MetaKVPairs { meta_prefix, meta_bound, meta_stage, meta_allow_priv } = match (MetaKVPairs::split(attr)) {
                Ok(meta) => meta,
                Err(e) => {
                    Span::call_site().error(e).emit();
                    return item
                }
            };
            let Some(meta_prefix ) = meta_prefix else { Span::call_site().error("No `prefix` value given" ).emit(); return item };
            let Some(meta_bound  ) = meta_bound  else { Span::call_site().error("No `bound` value given"  ).emit(); return item };
            let Some(meta_stage  ) = meta_stage  else { Span::call_site().error("No `stage` value given"  ).emit(); return item };
            let allow_priv = meta_allow_priv.is_some_and(|allow_priv| allow_priv);

            let struct_name = quote!{ #ident }.to_string();
            if (! struct_name.ends_with(&format!("{}Packet", quote!{ #meta_bound }.to_string()))) {
                Span::call_site().warning(format!("Packet `{}` does not have standard suffix `{}Packet`", struct_name, meta_bound)).emit();
            }

            PACKET_REFLECTION.lock().unwrap()
                .entry(meta_stage.to_string()).or_insert_with(|| HashMap::new())
                .entry(meta_bound.to_string()).or_insert_with(|| Vec::new())
                .push(Packet {
                    prefix      : quote!{ #meta_prefix }.to_string(),
                    struct_name
                });

            let (encode, decode) = match (fields) {

                Fields::Named(FieldsNamed { named, .. }) => {
                    let mut encode = Vec::new();
                    let mut decode = Vec::new();
                    for field @ Field { ident, vis, .. } in named {
                        if let Some(ident) = ident {
                            if (! allow_priv) { if let Visibility::Public(_) = vis { } else {
                                Span::call_site().warning(format!("Packet `{}` field `{}` is not public", item_struct.ident, ident)).emit();
                            } }
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
                        if (! allow_priv) { if let Visibility::Public(_) = field.vis { } else {
                            Span::call_site().warning(format!("Packet `{}` field `{}` is not public", item_struct.ident, i)).emit();
                        } }
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
                #[derive(Debug, Clone)]
                #item2
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
            Span::call_site().error("Unsupported item").emit();
            item
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
