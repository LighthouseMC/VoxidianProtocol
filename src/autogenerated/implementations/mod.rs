pub mod damage_types;
pub mod biomes;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::value::{Colour, Identifier, Particle};

trait ConvertOption {
    fn convert_option(&self) -> TokenStream;
}

trait ConvertVec {
    fn convert_vec(&self) -> TokenStream;
}

impl<T: ToTokens> ConvertVec for Vec<T> {
    fn convert_vec(&self) -> TokenStream {
        let mut out = TokenStream::new();
        for x in self {
            out.extend(quote! { #x, });
        }
        quote! { vec![#out] }
    }
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

impl ToTokens for Colour {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        
        tokens.extend(quote! {
            Colour::new_from_raw_int(self.to_int())
        });
    }
}

impl ToTokens for Identifier {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let a = self.namespace.to_string();
        let b = self.path.to_string();
        tokens.extend(quote! {
            Identifier::new(#a, #b)
        });
    }
}

impl ToTokens for Particle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = &self.id;
        tokens.extend(quote! {
            Particle {
                id: #id
            }
        });
    }
}