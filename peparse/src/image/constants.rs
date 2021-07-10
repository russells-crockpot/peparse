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
        (HighEntropyVa, 0x0020, "Image can handle a high entropy 64-bit virtual address space."),
        (DynamicBase, 0x0040, "DLL can be relocated at load time."),
        (ForceIntegrity, 0x0080, "Code Integrity checks are enforced."),
        (NxCompat, 0x0100, "Image is NX compatible."),
        (NoIsolation, 0x0200, "Isolation aware, but do not isolate the image."),
        (NoSeh, 0x0400, "Does not use structured exception (SE) handling. No SE handler may be called in this image."),
        (NoBind, 0x0800, "Do not bind the image."),
        (Appcontainer, 0x1000, "Image must execute in an AppContainer."),
        (WdmDriver, 0x2000, "A WDM driver."),
        (GuardCf, 0x4000, "Image supports Control Flow Guard."),
        (TerminalServerAware, 0x8000, "Terminal Server aware."),
    ]
}

constants_enum! {
    name: WinCertRevision,
    doc: "",
    value_type: u16,
    items: [
        (Rev1, 0x0100, "Version 1, legacy version of the Win_Certificate structure. It is supported only for purposes of verifying legacy Authenticode signatures"),
        (Rev2, 0x0200, "Version 2 is the current version of the Win_Certificate structure."),
    ]
}

constants_enum! {
    name: WinCertType,
    doc: "",
    value_type: u16,
    items: [
        (X509, 0x0001, "bCertificate contains an X.509 Certificate"),
        (PkcsSignedData, 0x0002, "bCertificate contains a PKCS#7 SignedData structure"),
        (Reserved1, 0x0003, "Reserved"),
        (TsStackSigned, 0x0004, "Terminal Server Protocol Stack Certificate signing"),
    ]
}
