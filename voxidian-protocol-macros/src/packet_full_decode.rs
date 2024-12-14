#[proc_macro]
pub fn packet_full_decode(input : TokenStream) -> TokenStream {


    let MetaKVPairs { meta_prefix, meta_bound, meta_stage, meta_allow_priv } = match (MetaKVPairs::split(input)) {
        Ok(meta) => meta,
        Err(e) => {
            Span::call_site().error(e).emit();
            return quote!{ }.into();
        }
    };
    let None             = meta_prefix     else { Span::call_site().error("`prefix` value must not be given"     ).emit(); return TokenStream::new() };
    let None             = meta_allow_priv else { Span::call_site().error("`allow_priv` value must not be given" ).emit(); return TokenStream::new() };
    let Some(meta_bound) = meta_bound      else { Span::call_site().error("No `bound` value given"               ).emit(); return TokenStream::new() };
    let Some(meta_stage) = meta_stage      else { Span::call_site().error("No `stage` value given"               ).emit(); return TokenStream::new() };
    let meta_bound = quote!{ #meta_bound }.to_string();
    let meta_stage = quote!{ #meta_stage }.to_string();

    let by_stage = PACKET_REFLECTION.lock().unwrap();
    let Some(by_bound) = by_stage.get(&meta_stage) else {
        Span::call_site().error(format!("No packets defined in stage {}", meta_stage)).emit();
        return TokenStream::new()
    };
    let Some(packets) = by_bound.get(&meta_bound) else {
        Span::call_site().error(format!("No packets defined in stage {} bound {}", meta_stage, meta_bound)).emit();
        return TokenStream::new()
    };

    let ident = parse_str::<Ident>(&format!("{}{}Packets", meta_stage, meta_bound)).unwrap();

    let mut variants = Vec::new();
    let mut encode   = Vec::new();
    let mut decode   = Vec::new();
    for packet in packets {
        let struct_name = parse_str::<Ident>(&packet.struct_name).unwrap();
        let variant_name = parse_str::<Ident>(packet.struct_name.strip_suffix(&format!("{}Packet", meta_bound)).unwrap_or(&packet.struct_name)).unwrap();
        let packet_prefix = parse_str::<Expr>(&packet.prefix).unwrap();
        variants.push(quote!{ #variant_name( #struct_name ), });
        encode.push(quote!{ Self::#variant_name(packet) => PacketEncodeFull::encode_full(packet, buf, secret_cipher), });
        decode.push(quote!{ #packet_prefix => { Ok(Self::#variant_name(PacketDecode::decode(buf)?)) } });
    }


    quote!{
        #[derive(Debug)]
        pub enum #ident { #(#variants)* }
        impl PacketEncodeFull for #ident {
            fn encode_full(&self, buf : &mut PacketBuf, secret_cipher : &mut SecretCipher) -> Result<(), EncodeError> { match (self) {
                #(#encode)*
            } }
        }
        impl PacketDecodeFull for #ident {
            fn decode_partial(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
                let packetid = buf.read_decode::<VarInt>().unwrap().as_i32();
                match (packetid) {
                    #(#decode)*
                    _ => Err(DecodeError::UnknownPacketPrefix)
                }
            }
        }
    }.into()
}
