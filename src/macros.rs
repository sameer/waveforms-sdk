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

macro_rules! get_int {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            let mut val = 0;
            let res = $func($($arg,)* &mut val);
            if res != 0 { Ok(val) } else { Err(WaveFormsError::get()) }
        }
    };
}

macro_rules! get_float {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            let mut val = 0.;
            let res = $func($($arg,)* &mut val);
            if res != 0 { Ok(val) } else { Err(WaveFormsError::get()) }
        }
    };
}

macro_rules! get_bool {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            let mut val = 0;
            let res = $func($($arg,)* &mut val);
            if res != 0 { Ok(val != 0) } else { Err(WaveFormsError::get()) }
        }
    };
}

macro_rules! set_true {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            if $func($($arg,)* 1) != 0 { Ok(()) } else { Err(WaveFormsError::get()) }
        }
    };
}

macro_rules! set_false {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            if $func($($arg,)* 0) != 0 { Ok(()) } else { Err(WaveFormsError::get()) }
        }
    };
}

macro_rules! call {
    ($func: ident $($arg: expr),*) => {
        unsafe {
            let res = $func($($arg,)*);
            if res != 0 { Ok(()) } else { Err(WaveFormsError::get()) }
        }
    };
}

macro_rules! make_struct {
    ($(#[$struct_meta:meta])* $name:ident { $($field:ident : $ty: ty),* }) => {
        #[derive(Debug, PartialEq)]
        $(#[$struct_meta])*
        pub struct $name {
            $(
                pub $field: $ty,
            )*
        }
    }
}

macro_rules! enum_only {
    ($(#[$enum_meta:meta])* $name: ident $ty: ident {
        $(
            $(#[$field_meta:meta])*
            $field: ident => $value: expr
        ),*
    }) => {
        paste! {
            #[derive(Debug, PartialEq, Clone, Copy)]
            $(#[$enum_meta])*
            #[non_exhaustive]
            pub enum $name {
                $(
                    $(#[$field_meta])*
                    $field,
                )*
            }

            impl core::convert::TryFrom<$ty> for $name {
                type Error = WaveFormsError;
                fn try_from(x: $ty) -> Result<Self, WaveFormsError> {
                    match x {
                        $($value => Ok(Self::$field),)*
                        other => Err(crate::WaveFormsError {
                            reason: format!("WaveForms SDK returned `{}` which is not a known variant of {}", other, stringify!($name)),
                            error_code: crate::WaveFormsErrorCode::UnknownVariant
                        })
                    }
                 }
            }

            impl Into<$ty> for $name {
                fn into(self) -> $ty {
                    match self {
                        $(Self::$field => $value,)*
                    }
                }
            }
        }
    };
}

macro_rules! enum_and_support_bitfield {
    ($(#[$enum_meta:meta])* $name: ident $ty: ident {
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
            $(#[$enum_meta])*
            pub enum $name {
                $(
                    $(#[$field_meta])*
                    $field,
                )*
            }

            impl std::convert::TryFrom<$ty> for $name {
                type Error = WaveFormsError;
                fn try_from(x: $ty) -> Result<Self, WaveFormsError> {
                    match x {
                        $($value => Ok(Self::$field),)*
                        other => Err(crate::WaveFormsError {
                            reason: format!("WaveForms SDK returned `{}` which is not a known variant of {}", other, stringify!($name)),
                            error_code: crate::WaveFormsErrorCode::UnknownVariant
                        })
                    }
                 }
            }

            impl Into<$ty> for $name {
                fn into(self) -> $ty {
                    match self {
                        $(Self::$field => $value,)*
                    }
                }
            }
        }
    };
}

macro_rules! enum_getter_and_setter {
    ($(#[$field_meta:meta])* $name: ident $ty: ident $base: ident $($arg: expr),*) => {
        paste! {
            pub fn [<get_ $name:snake:lower>](&self) -> Result<$ty, WaveFormsError> {
                use core::convert::TryFrom;
                get_int!([<$base Get>] $(self.$arg),*).and_then($ty::try_from)
            }
            $(#[$field_meta])*
            pub fn [<set_ $name:snake:lower>] (&mut self, x: $ty) -> Result<(), WaveFormsError> {
                call!([<$base Set>] $(self.$arg,)* x.into())
            }
        }
    };
}

macro_rules! uom_getter_and_setter {
    ($(#[$field_meta:meta])* $name: ident $ty: ident< $unit: ident> $base: ident $($arg: expr),*) => {
        paste! {
            pub fn [<get_ $name:snake:lower>](&self) -> Result<$ty, WaveFormsError> {
                get_float!([<$base Get>] $(self.$arg),*).map(|x| $ty::new::<$unit>(x))
            }
            $(#[$field_meta])*
            pub fn [<set_ $name:snake:lower>] (&mut self, x: $ty) -> Result<(), WaveFormsError> {
                call!([<$base Set>] $(self.$arg,)* x.get::<$unit>())
            }
        }
    };
}

macro_rules! int_getter_and_setter {
    ($(#[$field_meta:meta])* $name: ident $ty: ident $base: ident $($arg: expr),*) => {
        paste! {
            pub fn [<get_ $name:snake:lower>](&self) -> Result<$ty, WaveFormsError> {
                get_int!([<$base Get>] $(self.$arg),*)
            }
            $(#[$field_meta])*
            pub fn [<set_ $name:snake:lower>] (&mut self, x: $ty) -> Result<(), WaveFormsError> {
                call!([<$base Set>] $(self.$arg,)* x)
            }
        }
    };
}
