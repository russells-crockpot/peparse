#![allow(unused_macros)]

#[macro_export]
macro_rules! constants_enum {
    (
        name: $name:ident,
        doc: $doc:literal,
        value_type: $type:ty,
        items: [ $(($const_name:ident, $const_val:literal, $const_desc:literal),)+ ]
        $(, @markers: $($marker:ident),+)?
    ) => {
        #[derive(PartialEq, Debug, Clone, Copy)]
        #[doc = $doc]
        pub enum $name {
            $(
                #[doc = $const_desc]
                $const_name = $const_val,
            )+
        }

        $($(impl $marker for $name {})+)?

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $( Self::$const_name => stringify!($const_name).fmt(f), )+
                }
            }
        }

        impl ::core::convert::TryFrom<$type> for $name {
            type Error = crate::Error;

            fn try_from(value: $type) -> ::core::result::Result<Self, Self::Error> {
                match value {
                    $( $const_val => Ok(Self::$const_name), )+
                    other => Err(
                        Self::Error::InvalidConstant {
                            value_given: other as u64,
                            constant_type: stringify!($name).into()
                        }),
                }
            }
        }

        impl<'s> ::std::convert::TryFrom<&::segsource::Segment<'s, u8>> for $name {
            type Error = crate::Error;

            fn try_from(segment: &::segsource::Segment<'s, u8>)
                -> ::std::result::Result<Self, Self::Error>
            {
                match segment.next_int::<$type>()? {
                    $( $const_val => Ok(Self::$const_name), )+
                    other => Err(
                        Self::Error::InvalidConstant {
                            value_given: other as u64,
                            constant_type: stringify!($name).into()
                        }),
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

        impl::std::convert::From<$type> for $name {

            fn from(value: $type) -> Self {
                Self::from_bits_truncate(value)
            }
        }

        impl<'s> ::std::convert::TryFrom<&::segsource::Segment<'s, u8>> for $name {
            type Error = crate::Error;

            fn try_from(segment: &::segsource::Segment<'s, u8>)
                -> ::std::result::Result<Self, Self::Error>
            {
                Ok(Self::from_bits_truncate(segment.next_int()?))
            }
        }
    };
}

#[macro_export]
macro_rules! parsable {
    () => {
        #[derive(::segsource::TryFromSegment)]
        #[try_from_item_type(u8)]
        #[try_from_error(::crate::error::Error)]
    }
}
