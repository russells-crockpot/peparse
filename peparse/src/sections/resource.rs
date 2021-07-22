use crate::{
    error::{Error, Result},
    util::next_different_sizes,
    Rva, Va,
};
use core::convert::TryFrom;
use segsource::{DataSegment, TryFromSegment};

impl_section_specifics! { Resource, "." }

#[derive(TryFromSegment, Debug, Clone)]
#[from_seg(error(crate::Error), also_needs(is_32_plus: bool))]
pub struct ResourceDirectory {
    /// Resource flags. This field is reserved for future use. It is currently set to zero.
    pub characteristics: u32,

    /// The time that the resource data was created by the resource compiler.
    pub time_date_stamp: u32,

    /// The major version number, set by the user.
    pub major_version: u16,

    /// The minor version number, set by the user.
    pub minor_version: u16,

    /// The number of directory entries immediately following the table that use strings to identify
    /// Type, Name, or Language entries (depending on the level of the table).
    pub number_of_name_entries: u16,

    /// The number of directory entries immediately following the Name entries that use numeric IDs
    /// for Type, Name, or Language entries.
    pub number_of_id_entries: u16,
}

#[derive(Debug, Clone)]
pub enum DirectoryEntryOffset {
    /// Address of a Resource Data entry (a leaf).
    DataEntry(u32),
    /// Address of another resource directory table (the next level down).
    Subdirectory(u32),
}

impl<'s> TryFrom<&DataSegment<'s>> for DirectoryEntryOffset {
    type Error = Error;

    fn try_from(segment: &DataSegment<'s>) -> Result<Self> {
        let value = segment.next_u32()?;
        if value & 0x80000000 == 0 {
            Ok(Self::DataEntry(value))
        } else {
            Ok(Self::Subdirectory(value & 0x7fffffff))
        }
    }
}

#[derive(TryFromSegment, Debug, Clone)]
#[from_seg(error(crate::Error))]
pub struct NameDirectoryEntry {
    /// The offset of a string that gives the Type, Name, or Language ID entry, depending on level
    /// of table.
    pub name_offset: u32,

    pub offset: DirectoryEntryOffset,
}

#[derive(TryFromSegment, Debug, Clone)]
#[from_seg(error(crate::Error))]
pub struct IdDirectoryEntry {
    /// A 32-bit integer that identifies the Type, Name, or Language ID entry.
    pub integer_id: u32,

    pub offset: DirectoryEntryOffset,
}

#[derive(TryFromSegment, Debug, Clone)]
#[from_seg(error(crate::Error))]
pub struct ResourceDirectoryString {
    /// The size of the string, not including length field itself.
    pub length: u16,
    #[from_seg(size(length), from_iter)]
    pub string: Vec<u8>,
}

#[derive(TryFromSegment, Debug, Clone)]
#[from_seg(error(crate::Error))]
pub struct DataEntry {
    /// The address of a unit of resource data in the Resource Data area.
    pub data_rva: Rva,

    /// The size, in bytes, of the resource data that is pointed to by the Data RVA field.
    pub size: u32,

    /// The code page that is used to decode code point values within the resource data. Typically,
    /// the code page would be the Unicode code page.
    pub codepage: u32,

    _reserved: u32,
}

constants_enum! {
    name: ResourceType,
    doc: "",
    value_type: u16,
    items: [
        (Cursor, 1, ""),
        (Bitmap, 2, ""),
        (Icon, 3, ""),
        (Menu, 4, ""),
        (Dialog, 5, ""),
        (String, 6, ""),
        (FontDir, 7, ""),
        (Font, 8, ""),
        (Accelerator, 9, ""),
        (RcData, 10, ""),
        (MessageTable, 11, ""),
        (GroupCursor, 12, ""),
        (GroupIcon, 14, ""),
        (Version, 16, ""),
        (DlgInclude, 17, ""),
        (PlugPlay, 19, ""),
        (Vxd, 20, ""),
        (AniCursor, 21, ""),
        (AniIcon, 22, ""),
        (Html, 23, ""),
        (Manifest, 24, ""),
    ]
}
