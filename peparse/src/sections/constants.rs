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
        (TYPE_NO_PAD, 0x00000008, "The section should not be padded to the next boundary. This flag is obsolete and is replaced by IMAGE_SCN_ALIGN_1BYTES. This is valid only for object files., 0x00000010, Reserved for future use."),
        (CNT_CODE, 0x00000020, "The section contains executable code."),
        (CNT_INITIALIZED_DATA, 0x00000040, "The section contains initialized data."),
        (CNT_UNINITIALIZED_DATA, 0x00000080, "The section contains uninitialized data."),
        (LNK_OTHER, 0x00000100, "Reserved for future use."),
        (LNK_INFO, 0x00000200, "The section contains comments or other information. The .drectve section has this type. This is valid for object files only., 0x00000400, Reserved for future use."),
        (LNK_REMOVE, 0x00000800, "The section will not become part of the image. This is valid only for object files."),
        (LNK_COMDAT, 0x00001000, "The section contains COMDAT data. For more information, see COMDAT Sections (Object Only). This is valid only for object files."),
        (GPREL, 0x00008000, "The section contains data referenced through the global pointer (GP)."),
        (ALIGN_1BYTES, 0x00100000, "Align data on a 1-byte boundary. Valid only for object files."),
        (ALIGN_2BYTES, 0x00200000, "Align data on a 2-byte boundary. Valid only for object files."),
        (ALIGN_4BYTES, 0x00300000, "Align data on a 4-byte boundary. Valid only for object files."),
        (ALIGN_8BYTES, 0x00400000, "Align data on an 8-byte boundary. Valid only for object files."),
        (ALIGN_16BYTES, 0x00500000, "Align data on a 16-byte boundary. Valid only for object files."),
        (ALIGN_32BYTES, 0x00600000, "Align data on a 32-byte boundary. Valid only for object files."),
        (ALIGN_64BYTES, 0x00700000, "Align data on a 64-byte boundary. Valid only for object files."),
        (ALIGN_128BYTES, 0x00800000, "Align data on a 128-byte boundary. Valid only for object files."),
        (ALIGN_256BYTES, 0x00900000, "Align data on a 256-byte boundary. Valid only for object files."),
        (ALIGN_512BYTES, 0x00A00000, "Align data on a 512-byte boundary. Valid only for object files."),
        (ALIGN_1024BYTES, 0x00B00000, "Align data on a 1024-byte boundary. Valid only for object files."),
        (ALIGN_2048BYTES, 0x00C00000, "Align data on a 2048-byte boundary. Valid only for object files."),
        (ALIGN_4096BYTES, 0x00D00000, "Align data on a 4096-byte boundary. Valid only for object files."),
        (ALIGN_8192BYTES, 0x00E00000, "Align data on an 8192-byte boundary. Valid only for object files."),
        (LNK_NRELOC_OVFL, 0x01000000, "The section contains extended relocations."),
        (MEM_DISCARDABLE, 0x02000000, "The section can be discarded as needed."),
        (MEM_NOT_CACHED, 0x04000000, "The section cannot be cached."),
        (MEM_NOT_PAGED, 0x08000000, "The section is not pageable."),
        (MEM_SHARED, 0x10000000, "The section can be shared in memory."),
        (MEM_EXECUTE, 0x20000000, "The section can be executed as code."),
        (MEM_READ, 0x40000000, "The section can be read."),
        (MEM_WRITE, 0x80000000, "The section can be written to."),
    ]
}
