use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;

#[repr(C, packed)]
pub struct ShopGoodInfo {
    pub id: u32,
    pub name: PaddedString<MAX_NAME_LENGTH>,
    pub stock: u32,
    pub max_stock: u32,
    pub cooldown: f32,
    pub cooldown_remaining: f32,
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<ShopGoodInfo>() == 120);
