use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;
const MAX_BUTTON_ART_LENGTH: usize = 100;

#[repr(u8)]
pub enum BuildQueueType {
    Research = 0,
    Unit = 1,
    Reviving = 2,
}

#[repr(C, packed)]
pub struct BuildQueueInfo {
    id: u32,
    name: PaddedString<MAX_NAME_LENGTH>,
    training_progress: u32,
    build_type: BuildQueueType,
    button_art: PaddedString<MAX_BUTTON_ART_LENGTH>,
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<BuildQueueInfo>() == 209);
