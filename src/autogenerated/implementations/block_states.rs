use quote::{quote, ToTokens};

use crate::value::BlockState;

impl ToTokens for BlockState {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let block_id = &self.id;
        let properties = &self.properties;
        let keys: Vec<String> = properties.iter().map(|x| x.0.clone()).collect();
        let values: Vec<String> = properties.iter().map(|x| x.1.clone()).collect();
        tokens.extend(quote! {
            BlockState {
                id: #block_id,
                properties: vec![
                    #(
                        (#keys.to_string(), #values.to_string()),
                    )*
                ]
            }
        });
    }
}