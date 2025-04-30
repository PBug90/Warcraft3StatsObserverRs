use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;
const MAX_BUTTON_ART_LENGTH: usize = 100;

#[repr(C, packed)]
pub struct UnitInfo {
    pub id: u32,
    pub name: PaddedString<MAX_NAME_LENGTH>,
    pub owner_id: u32,
    pub current_amount: u32,
    pub total_amount: u32,
    pub button_art: PaddedString<MAX_BUTTON_ART_LENGTH>,
    pub is_peon: bool,
    pub is_functional_peon: bool,
    pub damage_dealt: u32,
    pub damage_received: u32,
    pub healing_done: u32,
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<UnitInfo>() == 230);
