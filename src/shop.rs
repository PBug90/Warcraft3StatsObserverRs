use crate::shop_good::ShopGoodInfo;
use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;
const MAX_GOODS: usize = 12;

#[repr(C, packed)]
pub struct ShopInfo {
    pub id: u32,
    pub name: PaddedString<MAX_NAME_LENGTH>,
    pub owner_id: u32,
    pub goods_count: u32,
    pub goods: [ShopGoodInfo; MAX_GOODS],
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<ShopInfo>() == 1552);
