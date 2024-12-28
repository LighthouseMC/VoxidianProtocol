use super::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AttributeType {
    pub id : Identifier
}
