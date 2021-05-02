use std::os::raw::c_int;

#[derive(Debug)]
pub struct Protocols<'handle> {
    pub(crate) device_handle: c_int,
    pub(crate) phantom: std::marker::PhantomData<&'handle ()>,
}
