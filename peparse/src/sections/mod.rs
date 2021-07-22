macro_rules! impl_section_specifics {
    (
        $name:ident,
        $name_str:literal,
        $table:ident
     ) => {
        mod __section_specifics {
            use super::$table;
            use crate::{
                error::{Error, Result},
                sections::SectionInfo,
            };
            use ::core::{
                convert::TryFrom,
                ops::{Deref, DerefMut},
            };
            use ::custom_debug_derive::Debug;
            use ::segsource::{BytesSource, DataSegment, Source as _};

            #[derive(Debug)]
            pub struct $name {
                table: $table,
                #[debug(skip)]
                data: BytesSource,
            }

            impl<'s> TryFrom<(bool, DataSegment<'s>)> for $name {
                type Error = Error;
                fn try_from((is_32_plus, segment): (bool, DataSegment<'_>)) -> Result<Self> {
                    let table = $table::try_from((is_32_plus, &segment))?;
                    Ok(Self {
                        table,
                        data: BytesSource::from_segment(segment)?,
                    })
                }
            }

            impl SectionInfo for $name {
                #[inline]
                fn section_type_name() -> &'static str {
                    $name_str
                }

                #[inline]
                fn raw_data(&self) -> Result<DataSegment<'_>> {
                    Ok(self.data.all()?)
                }
            }

            impl Deref for $name {
                type Target = $table;

                #[inline]
                fn deref(&self) -> &Self::Target {
                    &self.table
                }
            }

            impl DerefMut for $name {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.table
                }
            }
        }
        pub use __section_specifics::*;
    };
    ($name:ident, $name_str:literal) => {
        paste! {
            impl_section_specifics! {
                [<$name Section>],
                $name_str,
                [<$name Directory>]
            }
        }
    };
}

pub mod debug;
pub mod export;
pub mod import;
pub mod pdata;
pub mod relocation;
pub mod resource;
pub mod tls;

use crate::{
    error::{Error, Result},
    util::{align, parse_utf8_string},
    PeHeader, Rva,
};
use custom_debug_derive::Debug;
use segsource::{BytesSource, DataSegment, Source, TryFromSegment};
use std::{cmp::min, convert::TryFrom, ops::Range};

pub trait SectionInfo {
    fn section_type_name() -> &'static str;

    #[inline]
    fn name(&self) -> &str {
        Self::section_type_name()
    }

    fn raw_data(&self) -> Result<DataSegment>;
}

constants_enum! {
    name: SectionNumber,
    doc: "",
    value_type: u16,
    items: [
        (Undefined, 0, "The symbol record is not yet assigned a section. A value of zero indicates that a reference to an external symbol is defined elsewhere. A value of non-zero is a common symbol with a size that is specified by the value."),
        (Absolute, 0xffff, "The symbol has an absolute (non-relocatable) value and is not an address."),
        (Debug, 0xfffe, "The symbol provides general type or debugging information but does not correspond to a section. Microsoft tools use this setting along with .file records (storage class FILE)."),
    ]
}

flags! {
    name: SectionFlags,
    doc: "",
    value_type: u32,
    items: [
        (TypeNoPad, 0x00000008, "The section should not be padded to the next boundary. This flag is obsolete and is replaced by IMAGE_SCN_ALIGN_1BYTES. This is valid only for object files., 0x00000010, Reserved for future use."),
        (CntCode, 0x00000020, "The section contains executable code."),
        (CntInitializedData, 0x00000040, "The section contains initialized data."),
        (CntUninitializedData, 0x00000080, "The section contains uninitialized data."),
        (LnkOther, 0x00000100, "Reserved for future use."),
        (LnkInfo, 0x00000200, "The section contains comments or other information. The .drectve section has this type. This is valid for object files only., 0x00000400, Reserved for future use."),
        (LnkRemove, 0x00000800, "The section will not become part of the image. This is valid only for object files."),
        (LnkComdat, 0x00001000, "The section contains COMDAT data. For more information, see COMDAT Sections (Object Only). This is valid only for object files."),
        (Gprel, 0x00008000, "The section contains data referenced through the global pointer (GP)."),
        (Align_1bytes, 0x00100000, "Align data on a 1-byte boundary. Valid only for object files."),
        (Align_2bytes, 0x00200000, "Align data on a 2-byte boundary. Valid only for object files."),
        (Align_4bytes, 0x00300000, "Align data on a 4-byte boundary. Valid only for object files."),
        (Align_8bytes, 0x00400000, "Align data on an 8-byte boundary. Valid only for object files."),
        (Align_16bytes, 0x00500000, "Align data on a 16-byte boundary. Valid only for object files."),
        (Align_32bytes, 0x00600000, "Align data on a 32-byte boundary. Valid only for object files."),
        (Align_64bytes, 0x00700000, "Align data on a 64-byte boundary. Valid only for object files."),
        (Align_128bytes, 0x00800000, "Align data on a 128-byte boundary. Valid only for object files."),
        (Align_256bytes, 0x00900000, "Align data on a 256-byte boundary. Valid only for object files."),
        (Align_512bytes, 0x00A00000, "Align data on a 512-byte boundary. Valid only for object files."),
        (Align_1024bytes, 0x00B00000, "Align data on a 1024-byte boundary. Valid only for object files."),
        (Align_2048bytes, 0x00C00000, "Align data on a 2048-byte boundary. Valid only for object files."),
        (Align_4096bytes, 0x00D00000, "Align data on a 4096-byte boundary. Valid only for object files."),
        (Align_8192bytes, 0x00E00000, "Align data on an 8192-byte boundary. Valid only for object files."),
        (LnkNrelocOvfl, 0x01000000, "The section contains extended relocations."),
        (MemDiscardable, 0x02000000, "The section can be discarded as needed."),
        (MemNotCached, 0x04000000, "The section cannot be cached."),
        (MemNotPaged, 0x08000000, "The section is not pageable."),
        (MemShared, 0x10000000, "The section can be shared in memory."),
        (MemExecute, 0x20000000, "The section can be executed as code."),
        (MemRead, 0x40000000, "The section can be read."),
        (MemWrite, 0x80000000, "The section can be written to."),
    ]
}

#[derive(TryFromSegment, Debug, Clone)]
#[from_seg(error(Error))]
pub struct SectionHeader {
    /// An 8-byte, null-padded UTF-8 encoded string. If the string is exactly 8 characters long,
    /// there is no terminating null. For longer names, this field contains a slash (/) that is
    /// followed by an ASCII representation of a decimal number that is an offset into the string
    /// table. Executable images do not use a string table and do not support section names longer
    /// than 8 characters. Long names in object files are truncated if they are emitted to an
    /// executable file.
    #[from_seg(parser(parse_utf8_string::<8>(&segment, true)))]
    pub name: String,

    /// The total size of the section when loaded into memory. If this value is greater than
    /// SizeOfRawData, the section is zero-padded. This field is valid only for executable images
    /// and should be set to zero for object files.
    pub virtual_size: u32,

    /// For executable images, the address of the first byte of the section relative to the image
    /// base when the section is loaded into memory. For object files, this field is the address of
    /// the first byte before relocation is applied; for simplicity, compilers should set this to
    /// zero. Otherwise, it is an arbitrary value that is subtracted from offsets during relocation.
    #[debug(format = "0x{:x}")]
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
    #[debug(format = "0x{:x}")]
    pub pointer_to_raw_data: u32,

    /// The file pointer to the beginning of relocation entries for the section. This is set to zero
    /// for executable images or if there are no relocations.
    #[debug(format = "0x{:x}")]
    pub pointer_to_relocations: u32,

    /// The file pointer to the beginning of line-number entries for the section. This is set to
    /// zero if there are no COFF line numbers. This value should be zero for an image because COFF
    /// debugging information is deprecated.
    #[debug(format = "0x{:x}")]
    pub pointer_to_linenumbers: u32,

    /// The number of relocation entries for the section. This is set to zero for executable images.
    pub number_of_relocations: u16,

    /// The number of line-number entries for the section. This value should be zero for an image
    /// because COFF debugging information is deprecated.
    pub number_of_linenumbers: u16,

    /// The flags that describe the characteristics of the section.
    pub characteristics: SectionFlags,
}

impl SectionHeader {
    fn get_read_size(&self, pe_header: &PeHeader) -> u32 {
        let file_alignment = pe_header.optional_header.windows_specific.file_alignment;
        let read_size = min(
            align(
                self.pointer_to_raw_data + self.size_of_raw_data,
                file_alignment,
            ) - self.get_aligned_pointer_to_raw(),
            align(self.size_of_raw_data, 0x100),
        );
        if self.virtual_size != 0 {
            min(read_size, align(self.virtual_size, 0x100))
        } else {
            read_size
        }
    }

    #[inline]
    fn get_aligned_pointer_to_raw(&self) -> u32 {
        self.pointer_to_raw_data & !0x1ff
    }

    #[inline]
    pub(crate) fn calc_overlay_offset(&self, pe_header: &PeHeader) -> u64 {
        self.get_read_size(pe_header) as u64 + self.get_aligned_pointer_to_raw() as u64
    }
}

#[derive(Debug)]
pub struct UnknownSection {
    name: String,
    #[debug(skip)]
    data: BytesSource,
}

impl UnknownSection {
    fn new(name: String, segment: DataSegment) -> Result<Self> {
        Ok(Self {
            name,
            data: BytesSource::from_segment(segment)?,
        })
    }
}

impl SectionInfo for UnknownSection {
    #[inline]
    fn section_type_name() -> &'static str {
        "UNKNOWN"
    }

    #[inline]
    fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    fn raw_data(&self) -> Result<DataSegment<'_>> {
        Ok(self.data.all()?)
    }
}

#[derive(Debug)]
pub enum AnySection {
    Debug(debug::DebugSection),
    Export(export::ExportSection),
    Import(import::ImportSection),
    Tls(tls::TlsSection),
    Resource(resource::ResourceSection),
    Relocation(relocation::RelocationSection),
    Unknown(UnknownSection),
}

impl SectionInfo for AnySection {
    #[inline]
    fn section_type_name() -> &'static str {
        "*"
    }

    #[inline]
    fn name(&self) -> &str {
        match self {
            Self::Debug(value) => value.name(),
            Self::Export(value) => value.name(),
            Self::Import(value) => value.name(),
            Self::Tls(value) => value.name(),
            Self::Resource(value) => value.name(),
            Self::Relocation(value) => value.name(),
            Self::Unknown(value) => value.name(),
        }
    }

    #[inline]
    fn raw_data(&self) -> Result<DataSegment<'_>> {
        match self {
            Self::Debug(value) => value.raw_data(),
            Self::Export(value) => value.raw_data(),
            Self::Import(value) => value.raw_data(),
            Self::Tls(value) => value.raw_data(),
            Self::Resource(value) => value.raw_data(),
            Self::Relocation(value) => value.raw_data(),
            Self::Unknown(value) => value.raw_data(),
        }
    }
}

impl<'s> TryFrom<(&SectionHeader, DataSegment<'s>, bool)> for AnySection {
    type Error = Error;

    fn try_from(
        (header, segment, is_32_plus): (&SectionHeader, DataSegment<'s>, bool),
    ) -> Result<Self> {
        Ok(if debug::DebugSection::section_type_name() == header.name {
            Self::Debug(debug::DebugSection::try_from((is_32_plus, segment))?)
        } else if export::ExportSection::section_type_name() == header.name {
            Self::Export(export::ExportSection::try_from((is_32_plus, segment))?)
        } else if import::ImportSection::section_type_name() == header.name {
            Self::Import(import::ImportSection::try_from((is_32_plus, segment))?)
        } else if tls::TlsSection::section_type_name() == header.name {
            Self::Tls(tls::TlsSection::try_from((is_32_plus, segment))?)
        } else if resource::ResourceSection::section_type_name() == header.name {
            Self::Resource(resource::ResourceSection::try_from((is_32_plus, segment))?)
        } else if relocation::RelocationSection::section_type_name() == header.name {
            Self::Relocation(relocation::RelocationSection::try_from((
                is_32_plus, segment,
            ))?)
        } else {
            Self::Unknown(UnknownSection::new(header.name.clone(), segment)?)
        })
    }
}

#[derive(Debug)]
pub struct Section {
    pub header: SectionHeader,
    pub info: AnySection,
    section_start: u32,
    section_end: u32,
}

impl Section {
    pub fn new<S: Source<Item = u8>>(
        source: &S,
        pe_header: &PeHeader,
        header: SectionHeader,
        (section_start, section_end): (u32, u32),
    ) -> Result<Self> {
        let image_base = pe_header.optional_header.windows_specific.image_base;
        let segment = source.get_n(
            header.virtual_address as usize + image_base as usize,
            header.virtual_size as usize,
        )?;
        let info = AnySection::try_from((&header, segment, pe_header.is_pe32_plus()))?;
        Ok(Self {
            header,
            info,
            section_start,
            section_end,
        })
    }

    #[inline]
    pub fn rva_to_offset(&self, rva: Rva) -> Option<u64> {
        if !self.contains_rva(rva) {
            None
        } else {
            Some(
                (rva as u64 - self.header.virtual_address as u64)
                    + self.header.pointer_to_raw_data as u64,
            )
        }
    }

    #[inline]
    pub fn contains_rva(&self, rva: Rva) -> bool {
        rva >= self.section_start && rva < self.section_end
    }
}
