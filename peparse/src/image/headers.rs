use super::constants::{DllCharacteristics, WindowsSubsystem};
use crate::{
    error::{Error, Result},
    util::next_different_sizes,
};
use segsource::TryFromSegment;

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct MzHeader {
    #[from_seg(error_if(
        signature != 0x5a4d,
        Error::InvalidHeaderMagic {
            expected: "0x5a4d".into(),
            received: format!("{:04x}", signature),
        }
    ))]
    signature: u16,
    extra_bytes: u16,
    pages: u16,
    relocation_items: u16,
    header_size: u16,
    min_allocation: u16,
    max_allocation: u16,
    initial_ss: u16,
    initial_sp: u16,
    checksum: u16,
    initial_ip: u16,
    initial_cs: u16,
    relocation_table: u16,
    overlay: u16,
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct OptionalHeader {
    //TODO add a way to parse the value, only if it equals a certain value.
    /// The unsigned integer that identifies the state of the image file. The most common number is
    /// 0x10B, which identifies it as a normal executable file. 0x107 identifies it as a ROM image,
    /// and 0x20B identifies it as a PE32+ executable.
    pub magic: u16,

    /// The linker major version number.
    pub linker_major_version: u8,

    /// The linker minor version number.
    pub linker_minor_version: u8,

    /// The size of the code (text) section, or the sum of all code sections if there are multiple
    /// sections.
    pub size_of_code: u32,

    /// The size of the initialized data section, or the sum of all such sections if there are
    /// multiple data sections.
    pub size_of_initialized_data: u32,

    /// The size of the uninitialized data section (BSS), or the sum of all such sections if there
    /// are multiple BSS sections.
    pub size_of_uninitialized_data: u32,

    /// The address of the entry point relative to the image base when the executable file is loaded
    /// into memory. For program images, this is the starting address. For device drivers, this is
    /// the address of the initialization function. An entry point is optional for DLLs. When no
    /// entry point is present, this field must be zero.
    pub address_of_entry_point: u32,

    /// The address that is relative to the image base of the beginning-of-code section when it is
    /// loaded into memory.
    pub base_of_code: u32,

    /// The address that is relative to the image base of the beginning-of-data section when it is
    /// loaded into memory.
    #[from_seg(if(magic == 0x20b))]
    base_of_data: Option<u32>,
}
#[derive(TryFromSegment)]
#[from_seg(error(crate::Error), also_needs(magic: u16))]
pub struct OptionalHeaderWindowsSpecificFields {
    /// The preferred address of the first byte of image when loaded into memory; must be a multiple
    /// of 64 K. The default for DLLs is 0x10000000. The default for Windows CE EXEs is 0x00010000.
    /// The default for Windows NT, Windows 2000, Windows XP, Windows 95, Windows 98, and Windows Me
    /// is 0x00400000.
    #[from_seg(parser(next_different_sizes::<u32, u64>(magic != 0x20b, &segment)))]
    pub image_base: u64,

    /// The alignment (in bytes) of sections when they are loaded into memory. It must be greater
    /// than or equal to FileAlignment. The default is the page size for the architecture.
    pub section_alignment: u32,

    /// The alignment factor (in bytes) that is used to align the raw data of sections in the image
    /// file. The value should be a power of 2 between 512 and 64 K, inclusive. The default is 512.
    /// If the SectionAlignment is less than the architecture's page size, then FileAlignment must
    /// match SectionAlignment.
    pub file_alignment: u32,

    /// The major version number of the required operating system.
    pub operating_system_major_version: u16,

    /// The minor version number of the required operating system.
    pub operating_system_minor_version: u16,

    /// The major version number of the image.
    pub image_major_version: u16,

    /// The minor version number of the image.
    pub image_minor_version: u16,

    /// The major version number of the subsystem.
    pub subsystem_major_version: u16,

    /// The minor version number of the subsystem.
    pub subsystem_minor_version: u16,

    /// Reserved, must be zero.
    _win_32_version_value: u32,

    /// The size (in bytes) of the image, including all headers, as the image is loaded in memory.
    /// It must be a multiple of SectionAlignment.
    pub size_of_image: u32,

    /// The combined size of an MS-DOS stub, PE header, and section headers rounded up to a multiple
    /// of FileAlignment.
    pub size_of_headers: u32,

    /// The image file checksum. The algorithm for computing the checksum is incorporated into
    /// IMAGHELP.DLL. The following are checked for validation at load time: all drivers, any DLL
    /// loaded at boot time, and any DLL that is loaded into a critical Windows process.
    pub check_sum: u32,

    /// The subsystem that is required to run this image. For more information.
    pub subsystem: WindowsSubsystem,

    /// For more information.
    pub dll_characteristics: DllCharacteristics,

    /// The size of the stack to reserve. Only SizeOfStackCommit is committed; the rest is made
    /// available one page at a time until the reserve size is reached.
    #[from_seg(parser(next_different_sizes::<u32, u64>(magic != 0x20b, &segment)))]
    pub size_of_stack_reserve: u64,

    /// The size of the stack to commit.
    #[from_seg(parser(next_different_sizes::<u32, u64>(magic != 0x20b, &segment)))]
    pub size_of_stack_commit: u64,

    /// The size of the local heap space to reserve. Only SizeOfHeapCommit is committed; the rest is
    /// made available one page at a time until the reserve size is reached.
    #[from_seg(parser(next_different_sizes::<u32, u64>(magic != 0x20b, &segment)))]
    pub size_of_heap_reserve: u64,

    /// The size of the local heap space to commit.
    #[from_seg(parser(next_different_sizes::<u32, u64>(magic != 0x20b, &segment)))]
    pub size_of_heap_commit: u64,

    /// Reserved, must be zero.
    _loader_flags: u32,

    /// The number of data-directory entries in the remainder of the optional header. Each describes
    /// a location and size.
    pub number_of_rva_and_sizes: u32,
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct DataDirectoryPointer {
    pub rva: u32,
    pub size: u32,
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct OptionalHeaderDataDirectories {
    /// The export table address and size.
    pub export_table: DataDirectoryPointer,

    /// The import table address and size.
    pub import_table: DataDirectoryPointer,

    /// The resource table address and size.
    pub resource_table: DataDirectoryPointer,

    /// The exception table address and size.
    pub exception_table: DataDirectoryPointer,

    /// The attribute certificate table address and size.
    pub certificate_table: DataDirectoryPointer,

    /// The base relocation table address and size.
    pub base_relocation_table: DataDirectoryPointer,

    /// The debug data starting address and size.
    pub debug: DataDirectoryPointer,

    /// Reserved, must be 0
    pub _architecture: u64,

    /// The RVA of the value to be stored in the global pointer register. The size member of this
    /// structure must be set to zero.
    pub global_ptr_rva: DataDirectoryPointer,

    /// The thread local storage (TLS) table address and size.
    pub tls_table: DataDirectoryPointer,

    /// The load configuration table address and size.
    pub load_config_table: DataDirectoryPointer,

    /// The bound import table address and size.
    pub bound_import: DataDirectoryPointer,

    /// The import address table address and size.
    pub iat: DataDirectoryPointer,

    /// The delay import descriptor address and size.
    pub delay_import_descriptor: DataDirectoryPointer,

    /// The CLR runtime header address and size.
    pub clr_runtime_header: DataDirectoryPointer,

    _reserved: u64,
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct AttributeCertificate {
    /// Specifies the length of the attribute certificate entry.
    pub dw_length: u32,

    /// Contains the certificate version number.
    pub w_revision: u16,

    /// Specifies the type of content in bCertificate.
    pub w_certificate_type: u16,

    /// Contains a certificate, such as an Authenticode signature.
    pub _todo: u8,
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct DelayLoadImport {
    _attributes: u32,

    /// The RVA of the name of the DLL to be loaded. The name resides in the read-only data section
    /// of the image.
    pub name: u32,

    /// The RVA of the module handle (in the data section of the image) of the DLL to be delay-
    /// loaded. It is used for storage by the routine that is supplied to manage delay-loading.
    pub module_handle: u32,

    /// The RVA of the delay-load import address table.
    pub delay_import_address_table: u32,

    /// The RVA of the delay-load name table, which contains the names of the imports that might
    /// need to be loaded. This matches the layout of the import name table.
    pub delay_import_name_table: u32,

    /// The RVA of the bound delay-load address table, if it exists.
    pub bound_delay_import_table: u32,

    /// The RVA of the unload delay-load address table, if it exists. This is an exact copy of the
    /// delay import address table. If the caller unloads the DLL, this table should be copied back
    /// over the delay import address table so that subsequent calls to the DLL continue to use the
    /// thunking mechanism correctly.
    pub unload_delay_import_table: u32,

    /// The timestamp of the DLL to which this image has been bound.
    pub time_stamp: u32,
}
