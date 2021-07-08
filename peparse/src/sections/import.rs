use crate::{error::Error, Rva};
use segsource::TryFromSegment;

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
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
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct ImportLookup {
    //TODO the have odd sizes...
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct HintOrName {
    //TODO the have odd sizes...
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct ImportAddress {
    //TODO
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
