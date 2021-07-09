use crate::{
    error::{Error, Result},
    Rva,
};
use core::convert::TryFrom;
use segsource::{DataSegment, Endidness, TryFromSegment};

//TODO add a common Function enum

#[derive(TryFromSegment, Debug, Clone)]
#[from_seg(error(crate::Error))]
pub struct Mips32Function {
    /// The VA of the corresponding function.
    pub begin_address: u32,

    /// The VA of the end of the function.
    pub end_address: u32,

    /// The pointer to the exception handler to be executed.
    pub exception_handler: u32,

    /// The pointer to additional information to be passed to the handler.
    pub handler_data: u32,

    /// The VA of the end of the function's prolog.
    pub prolog_end_address: u32,
}

pub struct ArmFunction {
    /// The VA of the corresponding function.
    pub begin_address: u32,

    /// The number of instructions in the function's prolog.
    pub prolog_length: u8,

    pub function_length: u32,

    pub is_32_bit: bool,

    pub has_exception_handler: bool,
}

impl<'s> TryFrom<&DataSegment<'s>> for ArmFunction {
    type Error = Error;

    fn try_from(segment: &DataSegment<'s>) -> Result<Self> {
        let begin_address = segment.next_u32()?;
        let mut remaining = segment.next_u32()?;
        let has_exception_handler = remaining & 1 == 1;
        remaining >>= 1;
        let is_32_bit = remaining & 1 == 1;
        remaining >>= 1;
        let function_length = remaining & 0x3fffff;
        remaining >>= 22;
        Ok(Self {
            begin_address,
            prolog_length: remaining as u8,
            function_length,
            is_32_bit,
            has_exception_handler,
        })
    }
}

pub struct X64Function {
    /// The RVA of the corresponding function.
    pub begin_address: Rva,

    /// The RVA of the end of the function.
    pub end_address: Rva,

    /// The RVA of the unwind information.
    pub unwind_information: Rva,
}
