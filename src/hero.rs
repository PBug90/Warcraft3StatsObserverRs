use crate::ability::AbilityInfo;
use crate::item::ItemInfo;
use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;
const MAX_BUTTON_ART_LENGTH: usize = 100;
const MAX_ABILITIES: usize = 24;
const MAX_ITEMS: usize = 6;

#[repr(C, packed)]
pub struct HeroInfo {
    pub id: u32,
    pub name: PaddedString<MAX_NAME_LENGTH>,
    pub button_art: PaddedString<MAX_BUTTON_ART_LENGTH>,
    pub level: u32,
    pub experience: u32,
    pub level_up_experience: u32,
    pub hit_points: u32,
    pub max_hit_points: u32,
    pub mana_points: u32,
    pub max_mana_points: u32,
    pub damage_dealt: u32,
    pub damage_received: u32,
    pub self_damage: u32,
    pub pick_order: u32,
    pub healing_done: u32,
    pub number_of_deaths: u32,
    pub total_kills: u32,
    pub self_kills: u32,
    pub hero_kills: u32,
    pub building_kills: u32,
    pub time_alive_ms: u32,
    pub ability_count: u32,
    pub ability_info: [AbilityInfo; MAX_ABILITIES],
    pub item_count: u32,
    pub items: [ItemInfo; MAX_ITEMS],
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<HeroInfo>() == 5420);
