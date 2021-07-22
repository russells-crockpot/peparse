use crate::{
    coff::CoffFileHeader,
    error::{Error, Result},
    image::{DataDirectoryPointer, MzHeader, OptionalHeader},
    sections::{Section, SectionHeader},
    util::iter_to_result,
    Rva, Va,
};
use bytes::Bytes;
use custom_debug_derive::Debug;
use segsource::{
    BytesSource, DataSegment, Endidness, MappedFileSource, Source as _, TryFromSegment,
    U8Source as _,
};
use std::path::Path;
use std::{convert::TryFrom, mem, ops::Range};

#[derive(TryFromSegment, Debug, Clone)]
#[from_seg(error(crate::Error))]
pub struct PeHeader {
    pub ms_dos_header: MzHeader,

    #[from_seg(
        move_to(ms_dos_header.pe_offset as usize),
        error_if(
            _pe_signature != 0x00004550,
            Error::InvalidHeaderMagic {
                expected: "0x00004550".into(),
                received: format!("0x{:08x}", _pe_signature),
            }
    ))]
    _pe_signature: u32,

    #[from_seg(mut)]
    pub coff_header: CoffFileHeader,

    #[from_seg(
        parser(coff_header.optional_header.take().unwrap()),
        try=as_is,
        mut
    )]
    pub optional_header: OptionalHeader,

    #[from_seg(size(coff_header.number_of_sections), parse_each)]
    section_headers: Vec<SectionHeader>,
}

impl PeHeader {
    #[inline]
    pub fn is_pe32_plus(&self) -> bool {
        self.optional_header.is_pe32_plus()
    }
}

#[derive(Debug)]
pub struct PeFile {
    pub header: PeHeader,
    pub sections: Vec<Section>,
    pub file_size: u64,
    #[debug(skip)]
    pub overlay: BytesSource,
}

impl PeFile {
    #[inline]
    pub fn is_pe32_plus(&self) -> bool {
        self.header.is_pe32_plus()
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file_md = path.as_ref().metadata()?;
        //TODO account for non-native Endidness
        let mut source = MappedFileSource::from_file(path, Endidness::native())?;
        let mut header = PeHeader::try_from(&source.all()?)?;
        let is_pe32_plus = header.is_pe32_plus();
        let mut section_headers = mem::take(&mut header.section_headers);
        let mut overlay_offset = 0;
        for sec_header in &section_headers {
            let maybe_offset = sec_header.calc_overlay_offset(&header);
            if overlay_offset < maybe_offset {
                overlay_offset = maybe_offset;
            }
        }
        let overlay = BytesSource::from_bytes(
            if overlay_offset >= file_md.len() {
                Bytes::new()
            } else {
                Bytes::copy_from_slice(source.all_after(overlay_offset as usize)?.as_ref())
            },
            source.endidness(),
        )?;
        source.change_initial_offset(header.optional_header.windows_specific.image_base as usize);
        let mut section_ranges: Vec<(u32, u32)> = Vec::with_capacity(section_headers.len());
        for (i, sec_header) in section_headers.iter().enumerate() {
            let range = (sec_header.virtual_address, 0);
            if i != 0 {
                section_ranges[i - 1].1 = sec_header.virtual_address;
            }
            section_ranges.push(range);
        }
        let sections =
            iter_to_result(section_headers.into_iter().zip(section_ranges).map(
                |(sec_header, sec_range)| Section::new(&source, &header, sec_header, sec_range),
            ))?
            .collect();
        Ok(Self {
            header,
            sections,
            overlay,
            file_size: file_md.len(),
        })
    }

    pub fn rva_to_offset(&self, rva: Rva) -> Result<u64> {
        if let Some(section) = self.get_containing_section(rva) {
            Ok(section.rva_to_offset(rva).unwrap())
        } else {
            Err(Error::InvalidRva { rva })
        }
    }

    pub fn get_containing_section(&self, rva: Rva) -> Option<&Section> {
        for section in &self.sections {
            if section.contains_rva(rva) {
                return Some(section);
            }
        }
        None
    }
}
