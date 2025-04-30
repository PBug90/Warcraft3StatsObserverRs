use std::marker::PhantomData;
use winapi::shared::minwindef::LPVOID;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::memoryapi::{FILE_MAP_WRITE, MapViewOfFile, OpenFileMappingW};

use std::time::Duration;

use crate::game::ObserverGame;
use crate::player::PlayerInfo;
use crate::shop::ShopInfo;

const MAX_PLAYERS: usize = 28;
const MAX_SHOPS: usize = 999;

// Named tag where the warcraft 3 memory map lives
const OBSERVER_PATH: &str = r"War3StatsObserverSharedMemory";

#[repr(C, packed)]
pub struct ObserverData<'s> {
    pub version: u32,
    pub refresh_rate: u32,
    pub game: ObserverGame,
    pub players: [PlayerInfo; MAX_PLAYERS],
    pub shop_count: u32,
    pub shops: [ShopInfo; MAX_SHOPS],
    phantom: PhantomData<&'s ()>,
}

impl<'s> ObserverData<'s> {
    pub fn new() -> std::io::Result<&'s ObserverData<'s>> {
        Self::new_with_refresh_rate(Duration::from_millis(500))
    }

    pub fn new_with_refresh_rate(duration: Duration) -> std::io::Result<&'s ObserverData<'s>> {
        let mut path: Vec<u16> = OBSERVER_PATH.encode_utf16().collect();
        path.push(0);

        let mapping;
        let errno: i32;

        unsafe {
            mapping = OpenFileMappingW(FILE_MAP_WRITE, 0, path.as_ptr());
        };

        if mapping.is_null() {
            unsafe {
                errno = GetLastError() as i32;
            }

            return Err(std::io::Error::from_raw_os_error(errno));
        }

        let map_pointer: LPVOID;

        unsafe {
            map_pointer = MapViewOfFile(mapping, FILE_MAP_WRITE, 0, 0, 0);
        }

        if map_pointer.is_null() {
            unsafe {
                errno = GetLastError() as i32;
            }

            return Err(std::io::Error::from_raw_os_error(errno));
        }

        unsafe {
            let observer = &mut *(map_pointer as *mut ObserverData);
            observer.set_refresh_rate(duration);
            Ok(observer)
        }
    }

    pub fn disable(&mut self) {
        self.set_refresh_rate(Duration::ZERO);
    }

    pub fn set_refresh_rate(&mut self, duration: Duration) {
        self.refresh_rate = duration.as_millis() as u32;
    }
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<ObserverData>() == 181219642);
