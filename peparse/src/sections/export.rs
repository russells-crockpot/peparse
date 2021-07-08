use crate::{error::Error, Rva};
use segsource::TryFromSegment;

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct ExportDirectory {
    /// Reserved, must be 0.
    _export_flags: u32,

    /// The time and date that the export data was created.
    pub time_date_stamp: u32,

    /// The major version number. The major and minor version numbers can be set by the user.
    pub major_version: u16,

    /// The minor version number.
    pub minor_version: u16,

    /// The address of the ASCII string that contains the name of the DLL. This address is relative
    /// to the image base.
    pub name: Rva,

    /// The starting ordinal number for exports in this image. This field specifies the starting
    /// ordinal number for the export address table. It is usually set to 1.
    pub ordinal_base: u32,

    /// The number of entries in the export address table.
    pub address_table_entries: u32,

    /// The number of entries in the name pointer table. This is also the number of entries in the
    /// ordinal table.
    pub number_of_name_pointers: u32,

    /// The address of the export address table, relative to the image base.
    pub export_address_table: Rva,

    /// The address of the export name pointer table, relative to the image base. The table size is
    /// given by the Number of Name Pointers field.
    pub name_pointer: Rva,

    /// The address of the ordinal table, relative to the image base.
    pub ordinal_table: Rva,
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct ExportAddress {
    /// The address of the exported symbol when loaded into memory, relative to the image base. For
    /// example, the address of an exported function.
    pub export: Rva,

    /// The pointer to a null-terminated ASCII string in the export section. This string must be
    /// within the range that is given by the export table data directory entry. See Optional Header
    /// Data Directories (Image Only). This string gives the DLL name and the name of the export
    /// (for example, "MYDLL.expfunc") or the DLL name and the ordinal number of the export (for
    /// example, "MYDLL.#27").
    pub forwarder: Rva,
}
