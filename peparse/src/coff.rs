pub use crate::constants::coff::*;
use segsource::TryFromSegment;

#[derive(TryFromSegment)]
#[try_from_item_type(u8)]
#[try_from_error(crate::Error)]
pub struct CoffHeader {
    machine: Machine,
    number_of_sections: u16,
    time_date_stamp: u32,
    ptr_to_symbol_table: u32,
    size_of_optional_header: u16,
    characteristics: Characteristics,
}