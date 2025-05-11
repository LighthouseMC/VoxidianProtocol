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

    let mut try_intos = Vec::new();
    let packets_ident = parse_str::<Ident>(&format!("{}{}Packets", bound_str, stage_str)).unwrap();

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
            for other_stage in ["Handshake", "Status", "Login", "Config", "Play"] {
                if (bound_str == "S2C" && other_stage == "Handshake") { continue; }
                let bound_ident = parse_str::<Ident>(&bound_str.to_lowercase()).unwrap();
                let other_stage_ident = parse_str::<Ident>(&other_stage.to_lowercase()).unwrap();
                let try_into_ident = parse_str::<Ident>(&format!("TryInto{}{}Packets", bound_str, other_stage)).unwrap();
                let try_into_fn_ident = parse_str::<Ident>(&format!("try_into_{}_{}", bound_str.to_lowercase(), other_stage.to_lowercase())).unwrap();
                let try_into_ret_ident = parse_str::<Ident>(&format!("{}{}Packets", bound_str, other_stage)).unwrap();
                let result = if (stage_str == other_stage) { quote!{ Some(#packets_ident::#packet_id_tc(self)) } } else { quote!{ None } };
                try_intos.push(quote!{
                    impl crate::packet::#bound_ident::#other_stage_ident::#try_into_ident for #ident {
                        fn #try_into_fn_ident(self) -> Option<crate::packet::#bound_ident::#other_stage_ident::#try_into_ret_ident> { #result }
                    }
                });
            }
        }
    }

    for other_stage in ["Handshake", "Status", "Login", "Config", "Play"] {
        if (bound_str == "S2C" && other_stage == "Handshake") { continue; }
        let bound_ident = parse_str::<Ident>(&bound_str.to_lowercase()).unwrap();
        let other_stage_ident = parse_str::<Ident>(&other_stage.to_lowercase()).unwrap();
        let try_into_ident = parse_str::<Ident>(&format!("TryInto{}{}Packets", bound_str, other_stage)).unwrap();
        let try_into_fn_ident = parse_str::<Ident>(&format!("try_into_{}_{}", bound_str.to_lowercase(), other_stage.to_lowercase())).unwrap();
        let try_into_ret_ident = parse_str::<Ident>(&format!("{}{}Packets", bound_str, other_stage)).unwrap();
        let result = if (stage_str == other_stage) { quote!{ Some(self) } } else { quote!{ None } };
        try_intos.push(quote!{
            impl crate::packet::#bound_ident::#other_stage_ident::#try_into_ident for #packets_ident {
                fn #try_into_fn_ident(self) -> Option<crate::packet::#bound_ident::#other_stage_ident::#try_into_ret_ident> { #result }
            }
        });
    }

    let bound_ident  = parse_str::<Ident>(&format!("{}", bound_str)).unwrap();
    let boundt_ident = parse_str::<Ident>(&format!("Bound{}", bound_str)).unwrap();
    let stage_ident  = parse_str::<Ident>(&format!("{}", stage_str)).unwrap();
    let staget_ident = parse_str::<Ident>(&format!("Stage{}", stage_str)).unwrap();
    (quote!{
        #[derive(Debug, Clone)]
        pub enum #packets_ident { #( #fields )* }
        impl PrefixedPacketEncode for #packets_ident { fn encode_prefixed(&self, buf : &mut PacketWriter) -> Result<(), EncodeError> { match (self) { #(#encode)* } } }
        impl PrefixedPacketDecode for #packets_ident { fn decode_prefixed<'l>(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
            let packetid = buf.read_decode::<Var32>()?.as_i32() as u8;
            match (packetid) {
                #(#decode)*
                packetid => Err(DecodeError::UnknownPacketPrefix(packetid))
            }
        } }
        impl PacketMeta for #packets_ident {
            const BOUND  : Bound = Bound::#bound_ident;
            type  BoundT         = #boundt_ident;
            const STAGE  : Stage = Stage::#stage_ident;
            type  StageT         = #staget_ident;
        }
        #(#try_intos)*
    }).into()
}
