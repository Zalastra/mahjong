#![feature(associated_consts)]

pub trait IterableEnum<T> {
    const ENUM_VARIANTS: &'static [T];

    fn iter() -> ::std::slice::Iter<'static, T>;
}

#[macro_export]
macro_rules! iterable_enum {
    (
        $enum_name:ident {
            $meta_dec:meta
            $( $variant:ident, )+
        }
    ) => (
        #[$meta_dec]
        pub enum $enum_name { $( $variant, )* }

        impl $crate::IterableEnum<$enum_name> for $enum_name {
            const ENUM_VARIANTS: &'static [$enum_name] = &[ $( $enum_name::$variant, )* ];

            fn iter() -> ::std::slice::Iter<'static, $enum_name> {
                $enum_name::ENUM_VARIANTS.iter()
            }
        }
    )
}
