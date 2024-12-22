use crate::value::NbtElement;

pub trait IntoNbtElement {
    fn into_nbt(self) -> NbtElement;
}

impl IntoNbtElement for i8 {
    fn into_nbt(self) -> NbtElement {
        NbtElement::Byte(self)
    }
}

impl IntoNbtElement for i16 {
    fn into_nbt(self) -> NbtElement {
        NbtElement::Short(self)
    }
}

impl IntoNbtElement for i32 {
    fn into_nbt(self) -> NbtElement {
        NbtElement::Int(self)
    }
}

impl IntoNbtElement for i64 {
    fn into_nbt(self) -> NbtElement {
        NbtElement::Long(self)
    }
}

impl IntoNbtElement for f32 {
    fn into_nbt(self) -> NbtElement {
        NbtElement::Float(self)
    }
}

impl IntoNbtElement for f64 {
    fn into_nbt(self) -> NbtElement {
        NbtElement::Double(self)
    }
}

impl IntoNbtElement for String {
    fn into_nbt(self) -> NbtElement {
        NbtElement::String(self)
    }
}

impl<T: IntoNbtElement> IntoNbtElement for Vec<T> {
    fn into_nbt(self) -> NbtElement {
        NbtElement::List(self.into_iter().map(|x| x.into_nbt()).collect())
    }
}
