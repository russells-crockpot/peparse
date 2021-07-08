use crate::{error::Error, util::next_different_sizes, Va};
use segsource::TryFromSegment;

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error), also_needs(is_32_plus: bool))]
pub struct TlsDirectory {
    /// The starting address of the TLS template. The template is a block of data that is used to
    /// initialize TLS data. The system copies all of this data each time a thread is created, so it
    /// must not be corrupted. Note that this address is not an RVA; it is an address for which
    /// there should be a base relocation in the .reloc section.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub raw_data_start_va: u64,

    /// The address of the last byte of the TLS, except for the zero fill. As with the Raw Data
    /// Start VA field, this is a VA, not an RVA.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub raw_data_end_va: u64,

    /// The location to receive the TLS index, which the loader assigns. This location is in an
    /// ordinary data section, so it can be given a symbolic name that is accessible to the program.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub address_of_index: u64,

    /// The pointer to an array of TLS callback functions. The array is null-terminated, so if no
    /// callback function is supported, this field points to 4 bytes set to zero. For information
    /// about the prototype for these functions, see TLS Callback Functions.
    #[from_seg(parser(next_different_sizes::<u32, u64>(!is_32_plus, &segment)))]
    pub address_of_callbacks: u64,

    /// The size in bytes of the template, beyond the initialized data delimited by the Raw Data
    /// Start VA and Raw Data End VA fields. The total template size should be the same as the total
    /// size of TLS data in the image file. The zero fill is the amount of data that comes after the
    /// initialized nonzero data.
    pub size_of_zero_fill: u32,

    /// The four bits [23:20] describe alignment info. Possible values are those defined as
    /// IMAGE_SCN_ALIGN_*, which are also used to describe alignment of section in object files. The
    /// other 28 bits are reserved for future use.
    //TODO
    pub characteristics: u32,
}

constants_enum! {
    name: TlsCallbackFunction,
    doc: "Possible reserved parameter values for TLS callback function.",
    value_type: u32,
    items: [
        (ProcessAttach, 1, "A new process has started, including the first thread."),
        (ReadAttach, 2, "A new thread has been created. This notification sent for all but the first thread."),
        (ReadDetach, 3, "A thread is about to be terminated. This notification sent for all but the first thread."),
        (ProcessDetach, 0, "A process is about to terminate, including the original thread."),

    ]
}
