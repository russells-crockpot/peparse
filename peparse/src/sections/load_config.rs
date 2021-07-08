use crate::{error::Error, util::next_different_sizes, Va};
use segsource::TryFromSegment;

pub const IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_SHIFT: u32 = 28;

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error), also_needs(is_32_plus: bool))]
pub struct LoadConfig {
    /// Flags that indicate attributes of the file, currently unused.
    pub characteristics: u32,

    /// Date and time stamp value. The value is represented in the number of seconds that have
    /// elapsed since midnight (00:00:00), January 1, 1970, Universal Coordinated Time, according to
    /// the system clock. The time stamp can be printed by using the C runtime (CRT) time function.
    pub time_date_stamp: u32,

    /// Major version number.
    pub major_version: u16,

    /// Minor version number.
    pub minor_version: u16,

    /// The global loader flags to clear for this process as the loader starts the process.
    pub global_flags_clear: u32,

    /// The global loader flags to set for this process as the loader starts the process.
    pub global_flags_set: u32,

    /// The default timeout value to use for this process's critical sections that are abandoned.
    pub critical_section_default_timeout: u32,

    /// Memory that must be freed before it is returned to the system, in bytes.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub de_commit_free_block_threshold: u64,

    /// Total amount of free memory, in bytes.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub de_commit_total_free_threshold: u64,

    /// [x86 only] The VA of a list of addresses where the LOCK prefix is used so that they can be
    /// replaced with NOP on single processor machines.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub lock_prefix_table: u64,

    /// Maximum allocation size, in bytes.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub maximum_allocation_size: u64,

    /// Maximum virtual memory size, in bytes.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub virtual_memory_threshold: u64,

    /// Setting this field to a non-zero value is equivalent to calling SetProcessAffinityMask with
    /// this value during process startup (.exe only)
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub process_affinity_mask: u64,

    /// Process heap flags that correspond to the first argument of the HeapCreate function. These
    /// flags apply to the process heap that is created during process startup.
    pub process_heap_flags: u32,

    /// The service pack version identifier.
    pub csd_version: u16,

    /// Must be zero.
    pub reserved: u16,

    /// Reserved for use by the system.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub edit_list: u64,

    /// A pointer to a cookie that is used by Visual C++ or GS implementation.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub security_cookie: u64,

    /// [x86 only] The VA of the sorted table of RVAs of each valid, unique SE handler in the image.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub se_handler_table: u64,

    /// [x86 only] The count of unique handlers in the table.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub se_handler_count: u64,

    /// The VA where Control Flow Guard check-function pointer is stored.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub guard_cf_check_function_pointer: u64,

    /// The VA where Control Flow Guard dispatch-function pointer is stored.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub guard_cf_dispatch_function_pointer: u64,

    /// The VA of the sorted table of RVAs of each Control Flow Guard function in the image.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub guard_cf_function_table: u64,

    /// The count of unique RVAs in the above table.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub guard_cf_function_count: u64,

    /// Control Flow Guard related flags.
    pub guard_flags: GuardFlags,

    //TODO has a width of 12
    //pub code_integrity: u
    /// The VA where Control Flow Guard address taken IAT table is stored.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub guard_address_taken_iat_entry_table: u64,

    /// The count of unique RVAs in the above table.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub guard_address_taken_iat_entry_count: u64,

    /// The VA where Control Flow Guard long jump target table is stored.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub guard_long_jump_target_table: u64,

    /// The count of unique RVAs in the above table.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub guard_long_jump_target_count: u64,
}

flags! {
    name: GuardFlags,
    doc: "",
    value_type: u32,
    items: [
        (CfInstrumented, 0x00000100, "Module performs control flow integrity checks using system-supplied support."),
        (CfwInstrumented, 0x00000200, "Module performs control flow and write integrity checks."),
        (CfFunctionTablePresent, 0x00000400, "Module contains valid control flow target metadata."),
        (SecurityCookieUnused, 0x00000800, "Module does not make use of the /GS security cookie."),
        (ProtectDelayloadIat, 0x00001000, "Module supports read only delay load IAT."),
        (DelayloadIatInItsOwnSection, 0x00002000, "Delayload import table in its own .didat section (with nothing else in it) that can be freely reprotected."),
        (CfExportSuppressionInfoPresent, 0x00004000, "Module contains suppressed export information. This also infers that the address taken IAT table is also present in the load config."),
        (CfEnableExportSuppression, 0x00008000, "Module enables suppression of exports."),
        (CfLongjumpTablePresent, 0x00010000, "Module contains longjmp target information."),
        (CfFunctionTableSizeMask, 0xf0000000, "Mask for the subfield that contains the stride of Control Flow Guard function table entries (that is, the additional count of bytes per table entry)."),
    ]
}
