use super::constants::{
    relocations::CoffRelocationType, Characteristics, Machine, StorageClass, SymbolType,
};
use crate::{
    error::{Error, Result},
    image::OptionalHeader,
    sections::SectionNumber,
    util::next_different_sizes,
};
use core::convert::TryFrom;
use segsource::TryFromSegment;

#[derive(TryFromSegment, Debug, Clone)]
#[from_seg(error(crate::Error))]
pub struct CoffFileHeader {
    /// The number that identifies the type of target machine.
    pub machine: Machine,

    /// The number of sections. This indicates the size of the section table, which immediately
    /// follows the headers.
    pub number_of_sections: u16,

    /// The low 32 bits of the number of seconds since 00:00 January 1, 1970 (a C run-time
    /// time_t_value), which indicates when the file was created.
    pub time_date_stamp: u32,

    /// The file offset of the COFF symbol table, or zero if no COFF symbol table is present. This
    /// value should be zero for an image because COFF debugging information is deprecated.
    pub pointer_to_symbol_table: u32,

    /// The number of entries in the symbol table. This data can be used to locate the string table,
    /// which immediately follows the symbol table. This value should be zero for an image because
    /// COFF debugging information is deprecated.
    pub number_of_symbols: u32,

    /// The size of the optional header, which is required for executable files but not for object
    /// files. This value should be zero for an object file. For a description of the header format,
    /// see Optional Header (Image Only).
    pub size_of_optional_header: u16,

    /// The flags that indicate the attributes of the file.
    pub characteristics: Characteristics,

    #[from_seg(
        if(size_of_optional_header > 0),
        parser(OptionalHeader::try_from(&segment.next_n(size_of_optional_header as usize)?)))]
    pub optional_header: Option<OptionalHeader>,
}

#[derive(TryFromSegment, Debug, Clone)]
#[from_seg(error(crate::Error))]
pub struct CoffRelocations {
    virtual_address: u32,
    symbol_table_index: u32,
    relocation_type: u16,
}

impl CoffRelocations {
    pub fn get_type<T: CoffRelocationType>(&self) -> Result<T> {
        T::try_from(self.relocation_type)
    }
}

#[derive(TryFromSegment, Debug, Clone)]
#[from_seg(error(crate::Error))]
pub struct CoffLineNumber {
    /// This is a union of two fields: SymbolTableIndex and VirtualAddress. Whether SymbolTableIndex
    /// or RVA is used depends on the value of Linenumber.
    pub type_: u32,

    /// When nonzero, this field specifies a one-based line number. When zero, the Type field is
    /// interpreted as a symbol table index for a function.
    pub linenumber: u16,
}
