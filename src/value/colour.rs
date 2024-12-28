use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Colour(i32);

impl Colour {

    pub fn new_rgb(r : u8, g : u8, b : u8) -> Self { 
        Self((b as i32) | ((g as i32) << 8) | ((r as i32) << 16))
    }

}
impl Colour {

    pub fn to_int(&self) -> i32 {
        self.0
    }

}

