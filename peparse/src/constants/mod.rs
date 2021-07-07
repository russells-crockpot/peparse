pub(crate) mod coff;
pub(crate) mod relocations;
pub(crate) mod sections;

constants_enum! {
    name: WindowsSubsystem,
    doc: "",
    value_type: u16,
    items: [
        (Unknown, 0, "An unknown subsystem."),
        (Native, 1, "Device drivers and native Windows processes."),
        (WindowsGui, 2, "The Windows graphical user interface (GUI) subsystem."),
        (WindowsCui, 3, "The Windows character subsystem."),
        (Os2Cui, 5, "The OS/2 character subsystem."),
        (PosixCui, 7, "The Posix character subsystem."),
        (NativeWindows, 8, "Native Win9x driver."),
        (WindowsCeGui, 9, "Windows CE."),
        (EfiApplication, 10, "An Extensible Firmware Interface (EFI) application."),
        (EfiBootServiceDriver, 11, "An EFI driver with boot services."),
        (EfiRuntimeDriver, 12, "An EFI driver with run-time services."),
        (EfiRom, 13, "An EFI ROM image."),
        (Xbox, 14, "XBOX."),
        (WindowsBootApplication, 16, "Windows boot application."),
    ]
}

flags! {
    name: DllCharacteristics,
    doc: "",
    value_type: u16,
    items: [
        (HIGH_ENTROPY_VA, 0x0020, "Image can handle a high entropy 64-bit virtual address space."),
        (DYNAMIC_BASE, 0x0040, "DLL can be relocated at load time."),
        (FORCE_INTEGRITY, 0x0080, "Code Integrity checks are enforced."),
        (NX_COMPAT, 0x0100, "Image is NX compatible."),
        (NO_ISOLATION, 0x0200, "Isolation aware, but do not isolate the image."),
        (NO_SEH, 0x0400, "Does not use structured exception (SE) handling. No SE handler may be called in this image."),
        (NO_BIND, 0x0800, "Do not bind the image."),
        (APPCONTAINER, 0x1000, "Image must execute in an AppContainer."),
        (WDM_DRIVER, 0x2000, "A WDM driver."),
        (GUARD_CF, 0x4000, "Image supports Control Flow Guard."),
        (TERMINAL_SERVER_AWARE, 0x8000, "Terminal Server aware."),
    ]
}

//TODO might need to be a flag instead because it gets combined with another value.
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
