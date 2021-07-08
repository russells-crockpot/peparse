use crate::error::Error;
use segsource::TryFromSegment;

flags! {
    name: DegbugInfoType,
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
        (ExtendedDllCharacteristics, 20, "Extended DLL characteristics bits."),
    ]
}

//TODO not sure of the data type...
flags! {
    name: ExtendedDllCharacteristics,
    doc: "",
    value_type: u16,
    items: [
        (CetCompat, 0x0001, "Image is CET compatible."),
    ]
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct DebugDirectory {
    /// Reserved, must be zero.
    _characteristics: u32,

    /// The time and date that the debug data was created.
    pub time_date_stamp: u32,

    /// The major version number of the debug data format.
    pub major_version: u16,

    /// The minor version number of the debug data format.
    pub minor_version: u16,

    /// The format of debugging information. This field enables support of multiple debuggers.
    pub info_type: u32,

    /// The size of the debug data (not including the debug directory itself).
    pub size_of_data: u32,

    /// The address of the debug data when loaded, relative to the image base.
    pub address_of_raw_data: u32,

    /// The file pointer to the debug data.
    pub pointer_to_raw_data: u32,
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
///TODO the structure  uses bitfields. I need to check to see how those work better.
pub struct FrameFpo {
    ///offset 1st byte of function code
    ul_off_start: u32,

    ///# bytes in function
    cb_proc_size: u32,

    ///# bytes in locals/4
    cdw_locals: u32,

    ///# bytes in params/4
    cdw_params: u16,

    ///# bytes in prolog
    cb_prolog: u64,

    ///# regs saved
    cb_regs: u32,

    ///TRUE if SEH in func
    f_has_seh: u8,

    ///TRUE if EBP has been allocated
    f_use_bp: u8,

    ///reserved for future use
    reserved: u8,

    ///frame type
    cb_frame: u16,
}

//TODO figure out the format of these sections.
#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct DebugSubsection {}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct DebugSymbolSubsection {}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct DebugPrecompiledSubsection {}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct DebugTypeSubsection {}
