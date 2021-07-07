flags! {
    name: SectionFlags,
    doc: "",
    value_type: u32,
    items: [
        (Unknown, 0, "An unknown value that is ignored by all tools."),
        (Coff, 1, "The COFF debug information (line numbers, symbol table, and string table). This type of debug information is also pointed to by fields in the file headers."),
        (Codeview, 2, "The Visual C++ debug information."),
        (Fpo, 3, "The frame pointer omission (FPO) information. This information tells the debugger how to interpret nonstandard stack frames, which use the EBP register for a purpose other than as a frame pointer."),
        (Misc, 4, "The location of DBG file."),
        (Exception, 5, "A copy of .pdata section."),
        (Fixup, 6, "Reserved."),
        (OmapToSrc, 7, "The mapping from an RVA in image to an RVA in source image."),
        (OmapFromSrc, 8, "The mapping from an RVA in source image to an RVA in image."),
        (Borland, 9, "Reserved for Borland."),
        (Reserved10, 10, "Reserved."),
        (Clsid, 11, "Reserved."),
        (Repro, 16, "PE determinism or reproducibility."),

    ]
}

//TODO consider adding Extended DLL Characteristics (there's only one, so...)
