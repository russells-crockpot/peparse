use super::constants::SectionFlags;
use crate::error::Error;
use segsource::TryFromSegment;

#[derive(TryFromSegment, Debug, Clone)]
#[from_seg(error(crate::Error))]
pub struct SectionHeader {
    /// An 8-byte, null-padded UTF-8 encoded string. If the string is exactly 8 characters long,
    /// there is no terminating null. For longer names, this field contains a slash (/) that is
    /// followed by an ASCII representation of a decimal number that is an offset into the string
    /// table. Executable images do not use a string table and do not support section names longer
    /// than 8 characters. Long names in object files are truncated if they are emitted to an
    /// executable file.
    pub name: u64,

    /// The total size of the section when loaded into memory. If this value is greater than
    /// SizeOfRawData, the section is zero-padded. This field is valid only for executable images
    /// and should be set to zero for object files.
    pub virtual_size: u32,

    /// For executable images, the address of the first byte of the section relative to the image
    /// base when the section is loaded into memory. For object files, this field is the address of
    /// the first byte before relocation is applied; for simplicity, compilers should set this to
    /// zero. Otherwise, it is an arbitrary value that is subtracted from offsets during relocation.
    pub virtual_address: u32,

    /// The size of the section (for object files) or the size of the initialized data on disk (for
    /// image files). For executable images, this must be a multiple of FileAlignment from the
    /// optional header. If this is less than VirtualSize, the remainder of the section is zero-
    /// filled. Because the SizeOfRawData field is rounded but the VirtualSize field is not, it is
    /// possible for SizeOfRawData to be greater than VirtualSize as well. When a section contains
    /// only uninitialized data, this field should be zero.
    pub size_of_raw_data: u32,

    /// The file pointer to the first page of the section within the COFF file. For executable
    /// images, this must be a multiple of FileAlignment from the optional header. For object files,
    /// the value should be aligned on a 4-byte boundary for best performance. When a section
    /// contains only uninitialized data, this field should be zero.
    pub pointer_to_raw_data: u32,

    /// The file pointer to the beginning of relocation entries for the section. This is set to zero
    /// for executable images or if there are no relocations.
    pub pointer_to_relocations: u32,

    /// The file pointer to the beginning of line-number entries for the section. This is set to
    /// zero if there are no COFF line numbers. This value should be zero for an image because COFF
    /// debugging information is deprecated.
    pub pointer_to_linenumbers: u32,

    /// The number of relocation entries for the section. This is set to zero for executable images.
    pub number_of_relocations: u16,

    /// The number of line-number entries for the section. This value should be zero for an image
    /// because COFF debugging information is deprecated.
    pub number_of_linenumbers: u16,

    /// The flags that describe the characteristics of the section.
    pub characteristics: SectionFlags,
}
