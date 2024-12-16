#[proc_macro]
pub fn packet_full_decode(input : TokenStream) -> TokenStream {
    let source_file = syn::parse_file(&fs::read_to_string(Span::call_site().source_file().path()).unwrap()).unwrap();
    let mut fields = Vec::new();
    let mut encode = Vec::new();
    let mut decode = Vec::new();
    for item in source_file.items {
        if let Item::Struct(ItemStruct { attrs, ident, .. }) = item {
            if (attrs.iter().any(|attr| { if let Meta::List(MetaList { path, .. }) = &attr.meta && let Some(ident) = path.get_ident() && (ident.to_string() == "packet") { true } else { false } })) {
                let ident_str = ident.to_string();
                let field_ident = parse_str::<Ident>(ident_str.strip_suffix("S2CPacket").or_else(|| ident_str.strip_suffix("C2SPacket")).unwrap_or(&ident_str)).unwrap();
                fields.push(quote!{ #field_ident ( #ident ), });
                encode.push(quote!{ Self::#field_ident(packet) => PrefixedPacketEncode::encode_prefixed(packet, buf), });
                decode.push(quote!{ #ident::PREFIX => { Ok(Self::#field_ident(PacketDecode::decode(buf)?)) } });
            }
        }
    }
    let input2 : TokenStream2 = input.into();
    quote!{
        #[derive(Debug)]
        pub enum #input2 { #( #fields )* }
        impl PrefixedPacketEncode for #input2 {
            fn encode_prefixed(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> { match (self) {
                #(#encode)*
            } }
        }
        impl PrefixedPacketDecode for #input2 {
            fn decode_prefixed(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
                let packetid = buf.read_decode::<VarInt>().unwrap().as_i32() as usize;
                match (packetid) {
                    #(#decode)*
                    _ => Err(DecodeError::UnknownPacketPrefix)
                }
            }
        }
    }.into()
}
