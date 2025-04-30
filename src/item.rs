use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;
const MAX_BUTTON_ART_LENGTH: usize = 100;

#[repr(C, packed)]
pub struct ItemInfo {
    id: u32,
    name: PaddedString<MAX_NAME_LENGTH>,
    slot: u32,
    charges: u32,
    button_art: PaddedString<MAX_BUTTON_ART_LENGTH>,
}
// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<ItemInfo>() == 212);
