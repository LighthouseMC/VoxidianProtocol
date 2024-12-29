mod damage_types;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

trait ConvertOption {
    fn convert_option(&self) -> TokenStream;
}

impl<T: ToTokens> ConvertOption for Option<T> {
    fn convert_option(&self) -> TokenStream {
    
        match self {
            Some(x) => {
                let out = x.to_token_stream();
                quote! { Some(#out) }
            },
            None => quote! { None }
        }
    }
}