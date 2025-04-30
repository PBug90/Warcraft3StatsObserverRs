use std::fmt::Debug;

use crate::build_queue::BuildQueueInfo;
use crate::hero::HeroInfo;
use crate::player_item::PlayerItemInfo;
use crate::string_utils::PaddedString;
use crate::structure::StructureInfo;
use crate::unit::UnitInfo;
use crate::upgrade::UpgradeInfo;

const MAX_NAME_LENGTH: usize = 36;
const MAX_HEROES: usize = 999;
const MAX_STRUCTURES: usize = 999;
const MAX_UPGRADES: usize = 999;
const MAX_UNITS: usize = 999;
const MAX_UNITS_IN_QUEUE: usize = 999;
const MAX_ITEMS: usize = 999;
const MAX_UPKEEP_LEVELS: usize = 10;

#[derive(Debug)]
#[repr(u8)]
pub enum RacePreference {
    Human = 0x01,
    Orc = 0x02,
    Nightelf = 0x04,
    Undead = 0x08,
    Demon = 0x10,
    Random = 0x20,
    UserSelectable = 0x40,
}

#[derive(Debug)]
#[repr(u8)]
pub enum PlayerRace {
    Unknown = 0,
    Human = 1,
    Orc = 2,
    Undead = 3,
    NightElf = 4,
    Demon = 5,
    Last = 6,
    Other = 7,
    Creep = 8,
    Commoner = 9,
    Critter = 10,
    Naga = 11,
}

#[derive(Debug)]
#[repr(u8)]
pub enum PlayerType {
    Empty = 0,
    Player = 1,
    Computer = 2,
    Neutral = 3,
    Observer = 4,
    None = 5,
    Other = 6,
}

#[derive(Debug)]
#[repr(u8)]
pub enum PlayerGameResult {
    Victory = 0,
    Defeat = 1,
    Tie = 2,
    Neutral = 3,
}

#[derive(Debug)]
#[repr(u8)]
pub enum PlayerSlotState {
    Empty = 0,
    Playing = 1,
    Left = 2,
}

#[derive(Debug)]
#[repr(u8)]
pub enum AiDifficultyPreference {
    Newbie = 0,
    Normal = 1,
    Insane = 2,
}

#[repr(C, packed)]
pub struct PlayerInfo {
    pub name: PaddedString<MAX_NAME_LENGTH>,
    pub race_preference: RacePreference,
    pub player_race: PlayerRace,
    pub id: u8,
    pub team_index: u8,
    pub team_color: u8,
    pub player_type: PlayerType,
    pub handicap: u32,
    pub game_result: PlayerGameResult,
    pub slot_state: PlayerSlotState,
    pub ai_difficulty: AiDifficultyPreference,
    pub actions_per_minute: u32,
    pub real_time_actions_per_minute: u32,
    pub gold: u32,
    pub gold_mined: u32,
    pub gold_upkeep_lost: u32,
    pub gold_diversion_tax: u32,
    pub lumber: u32,
    pub lumber_mined: u32,
    pub lumber_upkeep_lost: u32,
    pub lumber_diversion_tax: u32,
    pub food_cap: u32,
    pub food_used: u32,
    pub hero_count: u32,
    pub heroes: [HeroInfo; MAX_HEROES],
    pub structure_count: u32,
    pub structures: [StructureInfo; MAX_STRUCTURES],
    pub upgrade_count: u32,
    pub upgrades: [UpgradeInfo; MAX_UPGRADES],
    pub unit_count: u32,
    pub units: [UnitInfo; MAX_UNITS],
    pub units_in_queue_count: u32,
    pub units_in_queue: [BuildQueueInfo; MAX_UNITS_IN_QUEUE],
    pub item_count: u32,
    pub items: [PlayerItemInfo; MAX_ITEMS],
    pub time_in_upkeep: [u32; MAX_UPKEEP_LEVELS],
}
// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<PlayerInfo>() == 6416738);
