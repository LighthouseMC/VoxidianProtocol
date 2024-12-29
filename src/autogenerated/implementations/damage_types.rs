use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::value::{DamageDifficultyScaling, DamageEffects, DamageType, DeathMessageType};

use super::ConvertOption;

impl ToTokens for DamageType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let message_id = &self.message_id;
        let scaling = self.scaling;
        let exhaustion = self.exhaustion;

        let effects = self.effects.convert_option();
        let death_message_type = self.death_message_type.convert_option();

        tokens.extend(quote! {
            DamageType {
                message_id: String::from(#message_id),
                scaling: #scaling,
                exhaustion: #exhaustion,
                effects: #effects,
                death_message_type: #death_message_type
            }
        });
    }
}

impl ToTokens for DamageEffects {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Burning => quote! { DamageEffects::Burning },
            Self::Drowning => quote! { DamageEffects::Drowning },
            Self::Hurt => quote! { DamageEffects::Hurt },
            Self::Thorns => quote! { DamageEffects::Thorns },
            Self::Freezing => quote! { DamageEffects::Freezing },
            Self::Poking => quote! { DamageEffects::Poking },

        });
    }
}

impl ToTokens for DamageDifficultyScaling {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Never => quote! { DamageDifficultyScaling::Never },
            Self::NonLivingPlayer => quote! { DamageDifficultyScaling::NonLivingPlayer },
            Self::Always => quote! { DamageDifficultyScaling::Always }
        });
    }
}

impl ToTokens for DeathMessageType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Default => quote! { DeathMessageType::Default },
            Self::FallVariants => quote! { DeathMessageType::FallVariants },
            Self::IntentionalGameDesign => quote! { DeathMessageType::IntentionalGameDesign }
        });
    }
}