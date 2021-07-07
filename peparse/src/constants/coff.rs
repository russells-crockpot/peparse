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

flags! {
    name: WinCertificateOptions,
    doc: "",
    value_type: u16,
    items: [
        (Revision10, 0x0100, "Version 1, legacy version of the Win_Certificate structure. It is supported only for purposes of verifying legacy Authenticode signatures"),
        (Revision20, 0x0200, "Version 2 is the current version of the Win_Certificate structure."),
        (RtTypeX509, 0x0001, "bCertificate contains an X.509 Certificate"),
        (RtTypePkcsSignedData, 0x0002, "bCertificate contains a PKCS#7 SignedData structure"),
        (RtTypeReserved1, 0x0003, "Reserved"),
        (RtTypeTsStackSigned, 0x0004, "Terminal Server Protocol Stack Certificate signing"),
    ]
}
