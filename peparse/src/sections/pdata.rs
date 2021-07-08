use crate::{error::Error, Rva, Va};
use segsource::TryFromSegment;

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct MipsFunction {
    /// The VA of the corresponding function.
    pub begin_address: Va,

    /// The VA of the end of the function.
    pub end_address: Va,

    /// The pointer to the exception handler to be executed.
    pub exception_handler: u32,

    /// The pointer to additional information to be passed to the handler.
    pub handler_data: u32,

    /// The VA of the end of the function's prolog.
    pub prolog_end_address: Va,
}

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct ArmFunction {
    /// The VA of the corresponding function.
    pub begin_address: Va,

    /// The number of instructions in the function's prolog.
    prolog_length: u8,
    // TODO Others have nonstandard byte lengths...
}

pub struct X64Function {
    /// The RVA of the corresponding function.
    pub begin_address: Rva,

    /// The RVA of the end of the function.
    pub end_address: Rva,

    /// The RVA of the unwind information.
    pub unwind_information: Rva,
}
