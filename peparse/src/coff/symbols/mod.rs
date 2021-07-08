use super::constants::{
    relocations::CoffRelocationType, Characteristics, Machine, StorageClass, SymbolType,
};
use crate::{
    error::{Error, Result},
    sections::SectionNumber,
    util::next_different_sizes,
};
use core::convert::TryFrom;
use segsource::TryFromSegment;

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct CoffSymbol {
    /// The name of the symbol, represented by a union of three structures. An array of 8 bytes is
    /// used if the name is not more than 8 bytes long.
    pub name: [u8; 8],

    /// The value that is associated with the symbol. The interpretation of this field depends on
    /// SectionNumber and StorageClass. A typical meaning is the relocatable address.
    pub value: u32,

    /// The signed integer that identifies the section, using a one-based index into the section
    /// table. Some values have special meaning, as defined in section 5.4.2, "Section Number
    /// Values."
    pub section_number: SectionNumber,

    /// A number that represents type. Microsoft tools set this field to 0x20 (function) or 0x0 (not
    /// a function).
    pub type_info: SymbolType,

    /// An enumerated value that represents storage class.
    pub storage_class: StorageClass,

    /// The number of auxiliary symbol table entries that follow this record.
    pub number_of_aux_symbols: u8,
}

//TODO add TryFrom<&DataSegment>
pub enum AuxSymbol {
    FunctionDef(FunctionDefSymbol),
    Bf(BfSymbol),
    Ef(EfSymbol),
    WekExternal(WekExternalSymbol),
    File(FileSymbol),
    Section(SectionSymbol),
    ClrTokenDef(ClrTokenDefSymbol),
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct FunctionDefSymbol {
    /// The symbol-table index of the corresponding .bf (begin function) symbol record.
    pub tag_index: u32,

    /// The size of the executable code for the function itself. If the function is in its own
    /// section, the SizeOfRawData in the section header is greater or equal to this field,
    /// depending on alignment considerations.
    pub total_size: u32,

    /// The file offset of the first COFF line-number entry for the function, or zero if none
    /// exists.
    pub pointer_to_line_number: u32,

    /// The symbol-table index of the record for the next function. If the function is the last in
    /// the symbol table, this field is set to zero.
    pub pointer_to_next_function: u32,

    #[from_seg(move_by(1))]
    _unused: u8,
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct BfSymbol {
    /// The actual ordinal line number (1, 2, 3, and so on) within the source file, corresponding to
    /// the .bf record.
    #[from_seg(move_by(4))]
    pub linenumber: u16,

    /// The symbol-table index of the next .bf symbol record. If the function is the last in the
    /// symbol table, this field is set to zero. It is not used for .ef records.
    pub pointer_to_next_function: u32,

    #[from_seg(move_by(1))]
    _unused: u8,
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct EfSymbol {
    /// The actual ordinal line number (1, 2, 3, and so on) within the source file, corresponding to
    /// the .bf record.
    #[from_seg(move_by(4))]
    pub linenumber: u16,
    #[from_seg(move_by(11))]
    _unused: u8,
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct WekExternalSymbol {
    /// The symbol-table index of sym2, the symbol to be linked if sym1 is not found.
    pub tag_index: u32,

    /// A value of IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY indicates that no library search for sym1
    /// should be performed.
    ///
    /// A value of IMAGE_WEAK_EXTERN_SEARCH_LIBRARY indicates that a library search for sym1 should
    /// be performed.
    ///
    /// A value of IMAGE_WEAK_EXTERN_SEARCH_ALIAS indicates that sym1 is an alias for sym2.
    //TODO create constant for this
    pub characteristics: u32,
    #[from_seg(move_by(9))]
    _unused: u8,
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct FileSymbol {
    /// An ANSI string that gives the name of the source file. This is padded with nulls if it is
    /// less than the maximum length.
    name: [u8; 18],
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct SectionSymbol {
    /// The size of section data; the same as SizeOfRawData in the section header.
    pub length: u32,

    /// The number of relocation entries for the section.
    pub number_of_relocations: u16,

    /// The number of line-number entries for the section.
    pub number_of_linenumbers: u16,

    //TODO add comdat constants.
    /// The checksum for communal data. It is applicable if the IMAGE_SCN_LNK_COMDAT flag is set in
    /// the section header.
    pub check_sum: u32,

    /// One-based index into the section table for the associated section. This is used when the
    /// COMDAT selection setting is 5.
    pub number: u16,

    /// The COMDAT selection number. This is applicable if the section is a COMDAT section.
    pub selection: u8,

    #[from_seg(move_by(2))]
    _unused: u8,
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct ClrTokenDefSymbol {
    //TODO add error_if
    /// Must be IMAGE_AUX_SYMBOL_TYPE_TOKEN_DEF (1).
    pub b_aux_type: u8,

    _reserved: u8,

    /// The symbol index of the COFF symbol to which this CLR token definition refers.
    pub symbol_table_index: u32,

    #[from_seg(move_by(11))]
    _unused: u8,
}

constants_enum! {
    name: Comdat,
    doc: "",
    value_type: u8,
    items: [
        (NoDuplicates, 1, "If this symbol is already defined, the linker issues a \"multiply defined symbol\" error."),
        (Any, 2, "Any section that defines the same COMDAT symbol can be linked; the rest are removed."),
        (SameSize, 3, "The linker chooses an arbitrary section among the definitions for this symbol. If all definitions are not the same size, a \"multiply defined symbol\" error is issued."),
        (ExactMatch, 4, "The linker chooses an arbitrary section among the definitions for this symbol. If all definitions do not match exactly, a \"multiply defined symbol\" error is issued."),
        (Associative, 5, "The section is linked if a certain other COMDAT section is linked. This other section is indicated by the Number field of the auxiliary symbol record for the section definition. This setting is useful for definitions that have components in multiple sections (for example, code in one and data in another), but where all must be linked or discarded as a set. The other section this section is associated with must be a COMDAT section, which can be another associative COMDAT section. An associative COMDAT section's section association chain can't form a loop. The section association chain must eventually come to a COMDAT section that doesn't have IMAGE_COMDAT_SELECT_ASSOCIATIVE set."),
        (Largest, 6, "The linker chooses the largest definition from among all of the definitions for this symbol. If multiple definitions have this size, the choice between them is arbitrary."),

    ]
}
