macro_rules! enum_builder {
    (
        #[repr($typ:ty)]
        $access:vis enum $name:ident {
            $( $arm:ident => $val:literal ),* $(,)?
        }
    ) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone,Copy, Eq, PartialEq, Debug)]
        $access enum $name {
            $($arm,)*
             Unknown($typ)
        }
        impl From<$typ> for $name {
            fn from(x: $typ) -> Self {
                match x {
                    $( $val => $name::$arm,)*
                    x => $name::Unknown(x),
                }
            }
        }
        impl From<$name> for $typ {
            fn from(enm: $name) -> Self {
                match enm {
                    $($name::$arm => $val ,)*
                    $name::Unknown(x) => x,
                }
            }
        }
    };
}
