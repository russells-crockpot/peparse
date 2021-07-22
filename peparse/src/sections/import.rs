use crate::{
    error::{Error, Result},
    Rva,
};
use segsource::{DataSegment, TryFromSegment};
use std::convert::TryFrom;

impl_section_specifics! { Import, ".idata" }

#[derive(TryFromSegment, Debug, Clone)]
#[from_seg(error(crate::Error), also_needs(is_32_plus: bool))]
pub struct ImportDirectory {
    /// The RVA of the import lookup table. This table contains a name or ordinal for each import.
    /// (The name "Characteristics" is used in Winnt.h, but no longer describes this field.)
    pub import_lookup_table: Rva,

    /// The stamp that is set to zero until the image is bound. After the image is bound, this field
    /// is set to the time/data stamp of the DLL.
    pub time_date_stamp: u32,

    /// The index of the first forwarder reference.
    pub forwarder_chain: u32,

    /// The address of an ASCII string that contains the name of the DLL. This address is relative
    /// to the image base.
    pub name: Rva,

    /// The RVA of the import address table. The contents of this table are identical to the
    /// contents of the import lookup table until the image is bound.
    pub import_address_table: Rva,

    #[from_seg(
        move_to(import_address_table as usize),
        also_pass(is_32_plus: bool),
        parse_each,
        while(value.as_ref().map(|v| !v.is_terminal()).unwrap_or(false))
    )]
    pub lookups: Vec<ImportLookupOrAddress>,
}

#[derive(Debug, Clone)]
pub struct ImportLookupOrAddress {
    /// If this bit is set, import by ordinal. Otherwise, import by name. Bit is masked as
    /// 0x80000000 for PE32, 0x8000000000000000 for PE32+.
    pub import_by_ordinal: bool,

    /// A 16-bit ordinal number. This field is used only if the Ordinal/Name Flag bit field is 1
    /// (import by ordinal). Bits 30-15 or 62-15 must be 0.
    pub ordinal_number: Option<u16>,

    /// A 31-bit RVA of a hint/name table entry. This field is used only if the Ordinal/Name Flag
    /// bit field is 0 (import by name). For PE32+ bits 62-31 must be zero.
    pub hint_name_table_rva: Option<Rva>,
}

impl ImportLookupOrAddress {
    fn is_terminal(&self) -> bool {
        !self.import_by_ordinal && self.hint_name_table_rva.as_ref().map(|v| *v == 0).unwrap()
    }
}

const IMPORT_BY_ORDINAL_MASK: u64 = 0x8000000000000000;
const ORDINAL_NUMBER_MASK: u64 = 0x7fff000000000000;
const NAME_TABLE_RVA_MASK: u64 = 0x7fffffff00000000;

impl<'s> TryFrom<(bool, &DataSegment<'s>)> for ImportLookupOrAddress {
    type Error = Error;

    fn try_from((is_32_plus, segment): (bool, &DataSegment<'s>)) -> Result<Self> {
        let num = if is_32_plus {
            segment.next_u64()?
        } else {
            (segment.next_u32()? as u64) << 32
        };
        let import_by_ordinal = (num & IMPORT_BY_ORDINAL_MASK) > 1;
        let mut ordinal_number = None;
        let mut hint_name_table_rva = None;
        if import_by_ordinal {
            ordinal_number = Some(((num & ORDINAL_NUMBER_MASK) >> 48) as u16)
        } else {
            hint_name_table_rva = Some(((num & NAME_TABLE_RVA_MASK) >> 32) as Rva)
        }
        Ok(Self {
            import_by_ordinal,
            ordinal_number,
            hint_name_table_rva,
        })
    }
}

constants_enum! {
    name: ImportType,
    doc: "",
    value_type: u16,
    items: [
        (Code, 0, "Executable code."),
        (Data, 1, "Data."),
        (Const, 2, "Specified as CONST in the .def file."),
    ]
}

constants_enum! {
    name: ImportNameType,
    doc: "",
    value_type: u16,
    items: [
        (Ordinal, 0, "The import is by ordinal. This indicates that the value in the Ordinal/Hint field of the import header is the import's ordinal. If this constant is not specified, then the Ordinal/Hint field should always be interpreted as the import's hint."),
        (Name, 1, "The import name is identical to the public symbol name."),
        (NameNoPrefix, 2, "The import name is the public symbol name, but skipping the leading ?, @, or optionally _."),
        (NameUndecorate, 3, "The import name is the public symbol name, but skipping the leading ?, @, or optionally _, and truncating at the first @."),
    ]
}
