use crate::Result;
use segsource::{marker::Integer, DataSegment, Endidness};

pub(crate) fn next_different_sizes<I1, I2>(parse_smaller: bool, segment: &DataSegment) -> Result<I2>
where
    I1: Integer,
    I2: Integer + From<I1>,
{
    if parse_smaller {
        Ok(I2::from(segment.next_int::<I1>()?))
    } else {
        Ok(segment.next_int::<I2>()?)
    }
}
