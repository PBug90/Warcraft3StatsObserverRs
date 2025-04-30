use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;

#[repr(C, packed)]
pub struct PlayerItemInfo {
    id: u32,
    name: PaddedString<MAX_NAME_LENGTH>,
    item_level: u32,
    collected: u32,
    purchased: u32,
    sold: u32,
    used: u32,
    destroyed: u32,
    damaged_dealt: u32,
    healing_done: u32,
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<PlayerItemInfo>() == 136);
