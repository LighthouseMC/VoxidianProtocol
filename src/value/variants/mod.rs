use serde::{Deserialize, Serialize};

mod wolf_variant;
pub use wolf_variant::*;
mod cat_variant;
pub use cat_variant::*;
mod pig_variant;
pub use pig_variant::*;
mod chicken_variant;
pub use chicken_variant::*;
mod cow_variant;
pub use cow_variant::*;
mod frog_variant;
pub use frog_variant::*;
mod wolf_sound_variant;
pub use wolf_sound_variant::*;

use super::NbtElement;

#[derive(Serialize, Deserialize)]
pub enum EntityModelType {
    Normal,
    Cold,
    Warm
}

impl EntityModelType {
    pub fn to_nbt(&self) -> NbtElement {
        match self {
            EntityModelType::Normal => NbtElement::String("normal".into()),
            EntityModelType::Cold => NbtElement::String("cold".into()),
            EntityModelType::Warm => NbtElement::String("warm".into()),
        }
    }
}