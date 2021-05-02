#[macro_export]
macro_rules! get_string {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            let mut buffer = [0i8; 32];
            let res = $func($($arg,)* &mut buffer);
            if res != 0 {
                Ok(CStr::from_ptr(buffer.as_ptr())
                    .to_str()
                    .unwrap()
                    .to_owned()
                    .to_string())
            } else {
                Err(WaveFormsError::get())
            }
        }
    };
}

#[macro_export]
macro_rules! get_int {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            let mut val = 0;
            let res = $func($($arg,)* &mut val);
            if res != 0 { Ok(val) } else { Err(WaveFormsError::get()) }
        }
    };
}

#[macro_export]
macro_rules! get_float {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            let mut val = 0.;
            let res = $func($($arg,)* &mut val);
            if res != 0 { Ok(val) } else { Err(WaveFormsError::get()) }
        }
    };
}

#[macro_export]
macro_rules! get_bool {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            let mut val = 0;
            let res = $func($($arg,)* &mut val);
            if res != 0 { Ok(val != 0) } else { Err(WaveFormsError::get()) }
        }
    };
}

#[macro_export]
macro_rules! set_true {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            if $func($($arg,)* 1) != 0 { Ok(()) } else { Err(WaveFormsError::get()) }
        }
    };
}

#[macro_export]
macro_rules! set_false {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            if $func($($arg,)* 0) != 0 { Ok(()) } else { Err(WaveFormsError::get()) }
        }
    };
}

#[macro_export]
macro_rules! call {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            let res = $func($($arg,)*);
            if res != 0 { Ok(()) } else { Err(WaveFormsError::get()) }
        }
    };
}

#[macro_export]
macro_rules! make_a_struct_and_getters {
    ($name:ident { $($field:ident : $ty: ty),* }) => {
        #[derive(Debug, PartialEq)]
        pub struct $name {
            $(
                $field: $ty,
            )*
        }

        paste! {
            impl $name {
                $(
                    pub fn [<$field>](&self) -> &$ty {
                        &self.$field
                    }
                )*
            }
        }
    }
}

#[macro_export]
macro_rules! enum_only {
    ($name: ident $ty: ident {
        $(
            $(#[$field_meta:meta])*
            $field: ident => $value: expr
        ),*
    }) => {
        paste! {
            #[derive(Debug, PartialEq, Clone, Copy)]
            #[non_exhaustive]
            pub enum $name {
                $(
                    $(#[$field_meta])*
                    $field,
                )*
                Other,
            }

            impl From<$ty> for $name {
                fn from(x: $ty) -> Self {
                    match x {
                        $($value => Self::$field,)*
                        _ => Self::Other
                    }
                 }
            }

            impl Into<$ty> for $name {
                fn into(self) -> $ty {
                    match self {
                        $(Self::$field => $value,)*
                        Self::Other => 0,
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! enum_and_support_bitfield {
    ($name: ident $ty: ident {
        $(
            $(#[$field_meta:meta])*
            $field: ident => $value: ident
        ),*
    }) => {
        paste! {
            #[derive(Debug, PartialEq)]
            #[non_exhaustive]
            pub struct [<Supported $name s>] {
                $(
                    $(#[$field_meta])*
                    pub [<$field:snake>]: bool
                ),*
            }

            impl From<c_int> for [<Supported $name s>] {
                fn from(x: c_int) -> Self {
                    Self {
                        $(
                            // exception: 0 is always true
                            [<$field:snake>]: (x & ((1 as c_int) << $value)) != 0 || $value == 0
                        ),*
                    }
                }
            }

            impl [<Supported $name s>] {
                /// See the supported choices as their enum variants
                pub fn as_enum_variants(&self) -> Vec<$name> {
                    let mut vec = Vec::default();
                    $(
                        if self.[<$field:snake>] {
                            vec.push($name::$field);
                        }
                    )*
                    vec
                }
            }

            #[non_exhaustive]
            #[derive(Debug, PartialEq, Clone, Copy)]
            pub enum $name {
                $(
                    $(#[$field_meta])*
                    $field,
                )*
                Other,
            }

            impl From<$ty> for $name {
                fn from(x: $ty) -> Self {
                    match x {
                        $($value => Self::$field,)*
                        _ => Self::Other
                    }
                 }
            }

            impl Into<$ty> for $name {
                fn into(self) -> $ty {
                    match self {
                        $(Self::$field => $value,)*
                        Self::Other => 0,
                    }
                }
            }
        }
    };
}
