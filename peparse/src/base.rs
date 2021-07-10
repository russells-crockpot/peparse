use crate::{
    coff::CoffFileHeader,
    error::{Error, Result},
    image::{DataDirectoryPointer, MzHeader, OptionalHeader},
};
use core::convert::TryFrom;
use segsource::{
    DataSegment, Endidness, MappedFileSource, Source as _, TryFromSegment, U8Source as _,
};
use std::path::Path;

#[derive(TryFromSegment, Debug, Clone)]
#[from_seg(error(crate::Error))]
pub struct PeFile {
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
}

// impl<S: U8Source> TryFrom<&S> for PeFile {
//     type Error: Error;
//
//     fn try_from(source: &S) -> Result<Self> {
//         <PeFile as TryFrom<DataSegment>>::try_from(source.all()?)
//     }
// }

impl PeFile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        //TODO account for non-native Endidness
        let source = MappedFileSource::from_file(path, Endidness::native())?;
        PeFile::try_from(&source.all()?)
    }
}
