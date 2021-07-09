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
