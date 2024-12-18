pub struct Colour {
    r : u8,
    g : u8,
    b : u8
}
impl Colour {

    pub fn new_rgb(r : u8, g : u8, b : u8) -> Self { Self { r, g, b } }

}
impl Colour {

    pub fn to_int(&self) -> i32 {
        (self.b as i32) | ((self.g as i32) << 8) | ((self.r as i32) << 16)
    }

}

