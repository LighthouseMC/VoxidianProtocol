use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse_str;
use inflector::Inflector;
use crate::PACKETS_DATA;

pub(crate) fn packet_full_decode_impl(input : TokenStream) -> TokenStream {
    let parts = format!("{}", input).split(" ").map(|part| {
        let Ok(part) = parse_str::<Ident>(part) else { panic!("expected an identifier") };
        part
    }).collect::<Vec<_>>();
    if (parts.len() != 2) { panic!("Expected 2 identifiers") }
    let bound     = parts[0].clone();
    let bound_str = bound.to_string();
    let stage     = parts[1].clone();
    let stage_str = stage.to_string();

    let mut fields = Vec::new();
    let mut encode = Vec::new();
    let mut decode = Vec::new();
    for (packet_id, packet) in  &PACKETS_DATA.packets {
        if (format!("{:?}", packet.stage) == stage_str && format!("{:?}", packet.bound) == bound_str) {
            let packet_id_tc = parse_str::<Ident>(&packet_id.split("/").last().unwrap().to_title_case().replace(" ", "")).unwrap();
            let ident        = parse_str::<Ident>(&format!("{}{}{}Packet", packet_id_tc, bound_str, stage_str)).unwrap();
            fields.push(quote!{ #packet_id_tc(#ident), });
            encode.push(quote!{ Self::#packet_id_tc(packet) => PrefixedPacketEncode::encode_prefixed(packet, buf), });
            decode.push(quote!{ #ident::PREFIX => { Ok(Self::#packet_id_tc(PacketDecode::decode(buf)?)) } });
        }
    }
    let ident = parse_str::<Ident>(&format!("{}{}Packets", bound_str, stage_str)).unwrap();
    (quote!{
        #[derive(Debug)]
        pub enum #ident { #( #fields )* }
        impl PrefixedPacketEncode for #ident { fn encode_prefixed(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> { match (self) { #(#encode)* } } }
        impl PrefixedPacketDecode for #ident { fn decode_prefixed(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
            let packetid = buf.read_decode::<VarInt>().unwrap().as_i32() as u8;
            match (packetid) {
                #(#decode)*
                packetid => Err(DecodeError::UnknownPacketPrefix(packetid))
            }
        } }
    }).into()
}
