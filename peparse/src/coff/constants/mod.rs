use crate::error::{Error, Result};
use core::convert::TryFrom;
use segsource::DataSegment;
pub mod relocations;

constants_enum! {
    name: Machine,
    doc: "",
    value_type: u16,
    items: [
        (Unknown, 0x0, "The content of this field is assumed to be applicable to any machine type"),
        (Am33, 0x1d3, "Matsushita AM33"),
        (Amd64, 0x8664, "x64"),
        (Arm, 0x1c0, "ARM little endian"),
        (Arm64, 0xaa64, "ARM64 little endian"),
        (ArmNt, 0x1c4, "ARM Thumb-2 little endian"),
        (Ebc, 0xebc, "EFI byte code"),
        (I386, 0x14c, "Intel 386 or later processors and compatible processors"),
        (Ia64, 0x200, "Intel Itanium processor family"),
        (M32r, 0x9041, "Mitsubishi M32R little endian"),
        (Mips16, 0x266, "MIPS16"),
        (MipsFpu, 0x366, "MIPS with FPU"),
        (MipsFpu16, 0x466, "MIPS16 with FPU"),
        (PowerPc, 0x1f0, "Power PC little endian"),
        (PowerPcFp, 0x1f1, "Power PC with floating point support"),
        (R4000, 0x166, "MIPS little endian"),
        (RiscV32, 0x5032, "RISC-V 32-bit address space"),
        (RiscV64, 0x5064, "RISC-V 64-bit address space"),
        (RiscV128, 0x5128, "RISC-V 128-bit address space"),
        (Sh3, 0x1a2, "Hitachi SH3"),
        (Sh3Dsp, 0x1a3, "Hitachi SH3 DSP"),
        (Sh4, 0x1a6, "Hitachi SH4"),
        (Sh5, 0x1a8, "Hitachi SH5"),
        (Thumb, 0x1c2, "Thumb"),
        (WceMipsV2, 0x169, "MIPS little-endian WCE v2"),
    ]
}

flags! {
    name: Characteristics,
    doc: "",
    value_type: u16,
    items: [
        (RELOCS_STRIPPED, 0x0001, "Image only, Windows CE, and Microsoft Windows NT and later. This indicates that the file does not contain base relocations and must therefore be loaded at its preferred base address. If the base address is not available, the loader reports an error. The default behavior of the linker is to strip base relocations from executable (EXE) files."),
        (EXECUTABLE_IMAGE, 0x0002, "Image only. This indicates that the image file is valid and can be run. If this flag is not set, it indicates a linker error."),
        (LINE_NUMS_STRIPPED, 0x0004, "COFF line numbers have been removed. This flag is deprecated and should be zero."),
        (LOCAL_SYMS_STRIPPED, 0x0008, "COFF symbol table entries for local symbols have been removed. This flag is deprecated and should be zero."),
        (AGGRESSIVE_WS_TRIM, 0x0010, "Obsolete. Aggressively trim working set. This flag is deprecated for Windows 2000 and later and must be zero."),
        (LARGE_ADDRESS_AWARE, 0x0020, "Application can handle > 2-GB addresses."),
        (BYTES_REVERSED_LO, 0x0080, "Little endian: the least significant bit (LSB) precedes the most significant bit (MSB) in memory. This flag is deprecated and should be zero."),
        (THIRTY_TWO_BIT_MACHINE, 0x0100, "Machine is based on a 32-bit-word architecture."),
        (DEBUG_STRIPPED, 0x0200, "Debugging information is removed from the image file."),
        (REMOVABLE_RUN_FROM_SWAP, 0x0400, "If the image is on removable media, fully load it and copy it to the swap file."),
        (NET_RUN_FROM_SWAP, 0x0800, "If the image is on network media, fully load it and copy it to the swap file."),
        (SYSTEM, 0x1000, "The image file is a system file, not a user program."),
        (DLL, 0x2000, "The image file is a dynamic-link library (DLL). Such files are considered executable files for almost all purposes, although they cannot be directly run."),
        (UP_SYSTEM_ONLY, 0x4000, "The file should be run only on a uniprocessor machine."),
        (BYTES_REVERSED_HI, 0x8000, "Big endian: the MSB precedes the LSB in memory. This flag is deprecated and should be zero."),
    ]
}

constants_enum! {
    name: StorageClass,
    doc: "",
    value_type: u8,
    items: [
        (EndOfFunction, 0xff, "A special symbol that represents the end of function, for debugging purposes."),
        (Null, 0, "No assigned storage class."),
        (Automatic, 1, "The automatic (stack) variable. The Value field specifies the stack frame offset."),
        (External, 2, "A value that Microsoft tools use for external symbols. The Value field indicates the size if the section number is IMAGE_SYM_UNDEFINED (0). If the section number is not zero, then the Value field specifies the offset within the section."),
        (Static, 3, "The offset of the symbol within the section. If the Value field is zero, then the symbol represents a section name."),
        (Register, 4, "A register variable. The Value field specifies the register number."),
        (ExternalDef, 5, "A symbol that is defined externally."),
        (Label, 6, "A code label that is defined within the module. The Value field specifies the offset of the symbol within the section."),
        (UndefinedLabel, 7, "A reference to a code label that is not defined."),
        (MemberOfStruct, 8, "The structure member. The Value field specifies the n th member."),
        (Argument, 9, "A formal argument (parameter) of a function. The Value field specifies the n th argument."),
        (StructTag, 10, "The structure tag-name entry."),
        (MemberOfUnion, 11, "A union member. The Value field specifies the n th member."),
        (UnionTag, 12, "The Union tag-name entry."),
        (TypeDefinition, 13, "A Typedef entry."),
        (UndefinedStatic, 14, "A static data declaration."),
        (EnumTag, 15, "An enumerated type tagname entry."),
        (MemberOfEnum, 16, "A member of an enumeration. The Value field specifies the n th member."),
        (RegisterParam, 17, "A register parameter."),
        (BitField, 18, "A bit-field reference. The Value field specifies the n th bit in the bit field."),
        (Block, 100, "A .bb (beginning of block) or .eb (end of block) record. The Value field is the relocatable address of the code location."),
        (Function, 101, "A value that Microsoft tools use for symbol records that define the extent of a function: begin function (.bf ), end function ( .ef ), and lines in function ( .lf ). For .lf records, the Value field gives the number of source lines in the function. For .ef records, the Value field gives the size of the function code."),
        (EndOfStruct, 102, "An end-of-structure entry."),
        (File, 103, "A value that Microsoft tools, as well as traditional COFF format, use for the source-file symbol record. The symbol is followed by auxiliary records that name the file."),
        (Section, 104, "A definition of a section (Microsoft tools use STATIC storage class instead)."),
        (WeakExternal, 105, "A weak external. For more information, see Auxiliary Format 3: Weak Externals."),
        (ClrToken, 107, "A CLR token symbol. The name is an ASCII string that consists of the hexadecimal value of the token. For more information, see CLR Token Definition (Object Only)."),
    ]
}

constants_enum! {
    name: TypeRepresentation,
    doc: "",
    value_type: u8,
    items: [
        (Null, 0, "No type information or unknown base type. Microsoft tools use this setting"),
        (Void, 1, "No valid type; used with void pointers and functions"),
        (Char, 2, "A character (signed byte)"),
        (Short, 3, "A 2-byte signed integer"),
        (Int, 4, "A natural integer type (normally 4 bytes in Windows)"),
        (Long, 5, "A 4-byte signed integer"),
        (Float, 6, "A 4-byte floating-point number"),
        (Double, 7, "An 8-byte floating-point number"),
        (Struct, 8, "A structure"),
        (Union, 9, "A union"),
        (Enum, 10, "An enumerated type"),
        (Moe, 11, "A member of enumeration (a specific value)"),
        (Byte, 12, "A byte; unsigned 1-byte integer"),
        (Word, 13, "A word; unsigned 2-byte integer"),
        (Uint, 14, "An unsigned integer of natural size (normally, 4 bytes)"),
        (Dword, 15, "An unsigned 4-byte integer"),
    ]
}

macro_rules! impl_sym_type {
    (
        name: $name:ident,
        doc: $doc:literal,
        items: [ $(($const_name:ident, $const_val:literal, $const_desc:literal),)+ ]
    ) => {
        #[doc=$doc]
        pub enum $name {
            $(
                #[doc = $const_desc]
                $const_name = $const_val,
            )+
        }

        impl TryFrom<u16> for $name {
            type Error = Error;

            #[allow(clippy::bad_bit_mask)]
            fn try_from(value: u16) -> Result<Self> {
                $(
                    if value & $const_val == $const_val {
                        Ok(Self::$const_name)
                    } else
                )+ {
                    Err(Error::InvalidConstant{
                        value_given: value as u64, constant_type: stringify!($name).into()
                    })
                }
            }
        }
    }
}

impl_sym_type! {
    name: SymbolBaseType,
    doc: "",
    items: [
        (Null, 0x0000, "No type information or unknown base type. Microsoft tools use this setting"),
        (Void, 0x0001, "No valid type; used with void pointers and functions"),
        (Char, 0x0002, "A character (signed byte)"),
        (Short, 0x0003, "A 2-byte signed integer"),
        (Int, 0x0004, "A natural integer type (normally 4 bytes in Windows)"),
        (Long, 0x0005, "A 4-byte signed integer"),
        (Float, 0x0006, "A 4-byte floating-point number"),
        (Double, 0x0007, "An 8-byte floating-point number"),
        (Struct, 0x0008, "A structure"),
        (Union, 0x0009, "A union"),
        (Enum, 0x000a, "An enumerated type"),
        (Moe, 0x000b, "A member of enumeration (a specific value)"),
        (Byte, 0x000c, "A byte; unsigned 1-byte integer"),
        (Word, 0x000d, "A word; unsigned 2-byte integer"),
        (Uint, 0x000e, "An unsigned integer of natural size (normally, 4 bytes)"),
        (Dword, 0x000f, "An unsigned 4-byte integer"),
    ]
}

impl_sym_type! {
    name: SymbolDataType,
    doc: "",
    items: [
        (Null, 0x0000, "No derived type; the symbol is a simple scalar variable."),
        (Pointer, 0x0100, "The symbol is a pointer to base type."),
        (Function, 0x0200, "The symbol is a function that returns a base type."),
        (Array, 0x0300, "The symbol is an array of base type."),
    ]
}

pub struct SymbolType {
    pub value: u16,
    pub base_type: SymbolBaseType,
    pub data_type: SymbolDataType,
}

impl<'s> TryFrom<&DataSegment<'s>> for SymbolType {
    type Error = Error;

    fn try_from(segment: &DataSegment<'s>) -> Result<Self> {
        let value = segment.next_u16()?;
        let base_type = SymbolBaseType::try_from(value)?;
        let data_type = SymbolDataType::try_from(value)?;
        Ok(Self {
            value,
            base_type,
            data_type,
        })
    }
}
