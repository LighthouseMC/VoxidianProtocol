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
pub struct ItemDoesntExistError;

impl fmt::Display for ItemDoesntExistError {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "item doesn't exist")
    }
}

impl Error for ItemDoesntExistError { }


impl FromStr for RegEntry<Item> {
    type Err = ItemDoesntExistError;
    fn from_str(s : &str) -> Result<Self, Self::Err> {
        Item::vanilla_registry().get_entry(&Identifier::from(s)).ok_or(ItemDoesntExistError)
    }
}
