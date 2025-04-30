use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 36;
const MAX_BUTTON_ART_LENGTH: usize = 100;

#[repr(C, packed)]
pub struct AbilityInfo {
    id: u32,
    name: PaddedString<MAX_NAME_LENGTH>,
    cooldown: f32,
    cooldown_remaining: f32,
    level: u32,
    button_art: PaddedString<MAX_BUTTON_ART_LENGTH>,
    is_hero_ability: u8,
    damage_dealt: u32,
    healing_done: u32,
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<AbilityInfo>() == 161);
