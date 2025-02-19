use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Colour(i32);

impl Colour {

    pub fn new_rgb(r : u8, g : u8, b : u8) -> Self {
        Self((b as i32) | ((g as i32) << 8) | ((r as i32) << 16))
    }

    pub fn new_from_raw_int(int: i32) -> Self {
        Colour(int)
    }

}
impl Colour {

    pub fn to_int(&self) -> i32 {
        self.0
    }

}
