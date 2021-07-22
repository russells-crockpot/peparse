use crate::{Result, Rva};
use segsource::{marker::Integer, DataSegment, Endidness};
use std::vec::IntoIter as VecIter;

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

pub(crate) fn parse_utf8_string<const N: usize>(
    segment: &DataSegment<'_>,
    strip_null: bool,
) -> Result<String> {
    let chars = segment.next_n_as_array::<N>()?;
    if strip_null {
        Ok(String::from_utf8(
            chars.iter().copied().filter(|v| *v != 0).collect(),
        )?)
    } else {
        Ok(String::from_utf8(Vec::from(chars))?)
    }
}

pub(crate) fn iter_to_result<V, I>(mut iter: I) -> Result<VecIter<V>>
where
    I: Iterator<Item = Result<V>>,
{
    let mut error = None;
    let tmp_vec: Vec<V> = iter
        .map(|r| match r {
            Ok(v) => Some(v),
            Err(e) => {
                error = Some(e);
                None
            }
        })
        .take_while(Option::is_some)
        .map(Option::unwrap)
        .collect();
    if let Some(error) = error {
        Err(error)
    } else {
        Ok(tmp_vec.into_iter())
    }
}

pub fn align(rva: Rva, align_to: u32) -> Rva {
    if rva % align_to != 0 {
        (rva + align_to - 1) & !(align_to - 1)
    } else {
        rva
    }
}
