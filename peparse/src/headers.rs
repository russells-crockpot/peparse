use crate::error::{Error, Result};
use segsource::TryFromSegment;

//#[derive(TryFromSegment)]
//#[from_seg(crate::Error)]
//struct PeOptionalHeader {
//}

#[derive(TryFromSegment)]
#[from_seg(crate::Error)]
struct OptionalHeaderStandardFields {
    magic: u16,
    major_linker_version: u8,
    minor_linker_version: u8,
    size_of_code: u32,
    size_of_initialized_data: u32,
    size_of_uninitialized_data: u32,
    address_of_entry_point: u32,
    base_of_code: u32,
    #[from_seg(if = magic == 0x20b)]
    base_of_data: u32,
}

pub struct OptionalHeader {}
