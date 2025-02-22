use super::*;
use core::str::FromStr;
use core::error::Error;

#[derive(Debug)]
pub struct Item {
    pub id : Identifier
}

impl Item {
    pub fn from_identifier(id: Identifier) -> Item {
        Item { id }
    }
}


#[derive(Debug)]
pub struct InvalidItemError;

impl fmt::Display for InvalidItemError {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid item")
    }
}

impl Error for InvalidItemError { }


impl FromStr for RegEntry<Item> {
    type Err = InvalidItemError;
    fn from_str(s : &str) -> Result<Self, Self::Err> {
        Item::vanilla_registry().get_entry(&Identifier::from(s)).ok_or(InvalidItemError)
    }
}
