pub trait IterableEnum {
    type T: IterableEnum;
    fn iter() -> ::std::slice::Iter<'static, Self::T>;
}

#[macro_export]
macro_rules! iterable_enum {
    (
        $enum_name:ident {
            #![$meta_dec:meta]
            $( $variant:ident, )+
        }
    ) => (
        #[$meta_dec]
        pub enum $enum_name { $( $variant, )* }

        impl $crate::IterableEnum for $enum_name {
            type T = Self;

            fn iter() -> ::std::slice::Iter<'static, $enum_name> {
                const ENUM_VARIANTS: &'static [$enum_name] = &[ $( $enum_name::$variant, )* ];
                ENUM_VARIANTS.iter()
            }
        }
    );
    (
        $enum_name:ident {
            $( $variant:ident, )+
        }
    ) => (
        pub enum $enum_name { $( $variant, )* }

        impl $crate::IterableEnum for $enum_name {
            type T = Self;

            fn iter() -> ::std::slice::Iter<'static, $enum_name> {
                const ENUM_VARIANTS: &'static [$enum_name] = &[ $( $enum_name::$variant, )* ];
                ENUM_VARIANTS.iter()
            }
        }
    )
}
