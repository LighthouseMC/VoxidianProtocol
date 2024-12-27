use super::*;
use voxidian_protocol_macros::import_item_registry_from_file;

#[derive(Debug)]
pub struct Item {
    pub id : Identifier
}

impl Item {
    pub fn from_identifier(id: Identifier) -> Item {
        Item { id }
    }

    pub fn vanilla_registry() -> Registry<Item> {
        import_item_registry_from_file!("./generated/items.json")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn vanilla_reg_test() {
        let rg = Item::vanilla_registry();
        assert!(rg.get(&Identifier::new("minecraft", "diamond")).is_some());
        assert!(rg.get(&Identifier::new("minecraft", "diamond_sword")).is_some());
        assert!(rg.get(&Identifier::new("minecraft", "dsjhkfhwejkew")).is_none());
        assert_eq!(rg.make_entry(&Identifier::new("minecraft", "conduit")).unwrap().id(), 643);
    }
}