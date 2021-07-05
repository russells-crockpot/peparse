#![allow(unused_macros)]

#[macro_export]
macro_rules! constants_enum {
    (
        name: $name:ident,
        doc: $doc:literal,
        value_type: $type:ty,
        items: [ $(($const_name:ident, $const_val:literal, $const_desc:literal),)+ ]
    ) => {
        #[doc = $doc]
        pub enum $name {
            $(
                #[doc = $const_desc]
                $const_name = $const_val,
            )+
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $( Self::$const_name => stringify!($const_name).fmt(f), )+
                }
            }
        }

        impl<'a> ::std::convert::TryFrom<&::segsource::Segment<'a, u8>> for $name {
            type Error = crate::Error;

            fn try_from(segment: &::segsource::Segment<'a, u8>)
                -> ::std::result::Result<Self, Self::Error>
            {
                match segment.next_int::<$type>()? {
                    $( $const_val => Ok(Self::$const_name), )+
                    other => Err(
                        Self::Error::InvalidConstant(other as u64, stringify!($name).into())),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! flags {
    (
        name: $name:ident,
        doc: $doc:literal,
        value_type: $type:ty,
        items: [ $(($const_name:ident, $const_val:literal, $const_desc:literal),)+ ]
    ) => {
        ::bitflags::bitflags! {
            #[doc = $doc]
            pub struct $name: $type {
                $(
                    #[doc = $const_desc]
                    const $const_name = $const_val;
                )+
            }
        }

        impl<'a> ::std::convert::TryFrom<&::segsource::Segment<'a, u8>> for $name {
            type Error = crate::Error;

            fn try_from(segment: &::segsource::Segment<'a, u8>)
                -> ::std::result::Result<Self, Self::Error>
            {
                Ok(Self::from_bits_truncate(segment.next_int()?))
            }
        }
    };
}
