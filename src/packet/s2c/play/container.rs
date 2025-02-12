use super::*;


#[packet("minecraft:s2c/play/open_screen")]
pub struct OpenScreenS2CPlayPacket {
    pub window : VarInt,
    pub kind   : ScreenWindowKind,
    pub title  : NbtText
}

#[packet_part(VarInt)]
#[derive(Copy, Eq)]
pub enum ScreenWindowKind {
    Generic9x1   = 0,
    Generic9x2   = 1,
    Generic9x3   = 2,
    Generic9x4   = 3,
    Generic9x5   = 4,
    Generic9x6   = 5,
    Generic3x3   = 6,
    Crafter3x3   = 7,
    Anvil        = 8,
    Beacon       = 9,
    BlastFurnace = 10,
    BrewingStand = 11,
    Crafting     = 12,
    Enchantment  = 13,
    Furnace      = 14,
    Grindstone   = 15,
    Hopper       = 16,
    Lectern      = 17,
    Loom         = 18,
    Merchant     = 19,
    ShulkerBox   = 20,
    Smithing     = 21,
    Smoker       = 22,
    Cartography  = 23,
    Stonecutter  = 24
}
impl ScreenWindowKind {

    /// The slot count of the container represented by the screen.
    /// This does not include the player inventory.
    pub fn container_slot_count(self) -> usize {
        match (self) {
            Self::Generic9x1   => 9,
            Self::Generic9x2   => 18,
            Self::Generic9x3   => 27,
            Self::Generic9x4   => 36,
            Self::Generic9x5   => 45,
            Self::Generic9x6   => 54,
            Self::Generic3x3   => 9,
            Self::Crafter3x3   => 9,
            Self::Anvil        => 3,
            Self::Beacon       => 1,
            Self::BlastFurnace => 3,
            Self::BrewingStand => 5,
            Self::Crafting     => 10,
            Self::Enchantment  => 2,
            Self::Furnace      => 3,
            Self::Grindstone   => 3,
            Self::Hopper       => 5,
            Self::Lectern      => 1,
            Self::Loom         => 4,
            Self::Merchant     => 3,
            Self::ShulkerBox   => 27,
            Self::Smithing     => 4,
            Self::Smoker       => 3,
            Self::Cartography  => 3,
            Self::Stonecutter  => 2
        }
    }

    /// Whether this screen includes the player's inventory.
    pub fn has_player_inventory(self) -> bool {
        self != Self::Lectern
    }

    /// The total slot count of the container represented by the screen.
    /// This includes the player inventory.
    pub fn full_slot_count(self) -> usize {
        self.container_slot_count() + (if (self.has_player_inventory()) { 36 } else { 0 })
    }

    /// Gets the screen slot index of a player's upper inventory slot.
    /// The upper inventory is the 27 non-hotbar slots.
    pub fn get_player_upper_index(self, player_upper_index : usize) -> Option<usize> {
        if (self.has_player_inventory() && player_upper_index < 27) {
            Some(self.container_slot_count() + player_upper_index)
        } else { None }
    }

    /// Gets the screen slot index of a player's hotbar slot.
    pub fn get_player_hotbar_index(self, player_hotbar_index : usize) -> Option<usize> {
        if (self.has_player_inventory() && player_hotbar_index < 9) {
            Some(self.container_slot_count() + 27 + player_hotbar_index)
        } else { None }
    }

    /// Gets the screen slot index of a player's inventory slot.
    pub fn get_player_inventory_index(self, player_inventory_index : usize) -> Option<usize> {
        if (! self.has_player_inventory()) {
            None
        } else if (player_inventory_index < 9) {
            Some(self.container_slot_count() + 27 + player_inventory_index)
        } else if (player_inventory_index < 36) {
            Some(self.container_slot_count() - 9 + player_inventory_index)
        } else { None }
    }

    /// Returns `true` if the given screen slot index is a player inventory slot.
    pub fn is_slot_index_player(self, screen_slot_index : usize) -> bool {
        if (self.has_player_inventory()) {
            let count = self.container_slot_count();
            (count..(count + 36)).contains(&screen_slot_index)
        } else { false }
    }

    /// Gets what kind of slot the given screen slot is.
    pub fn get_slot_index_group(self, screen_slot_index : usize) -> Option<ContainerSlotGroup> {
        let count = self.container_slot_count();
        if (self.has_player_inventory()) {
            if (screen_slot_index < count) {
                Some(ContainerSlotGroup::Container)
            } else if (screen_slot_index < count + 27) {
                Some(ContainerSlotGroup::PlayerUpper(screen_slot_index - count - 27))
            } else if (screen_slot_index < count + 36) {
                Some(ContainerSlotGroup::PlayerHotbar(screen_slot_index - count - 36))
            } else { None }
        } else if (screen_slot_index < count) {
            Some(ContainerSlotGroup::Container)
        } else { None }
    }

}


pub enum ContainerSlotGroup {
    PlayerHotbar(usize),
    PlayerUpper(usize),
    Container
}
