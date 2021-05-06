#[test]
fn version() {
    dbg!(crate::version());
}

#[cfg(feature = "local_tests")]
/// These can only be run on a system with an attached device.
/// They must be explicitly enabled
mod local_tests {
    #[test]
    fn enumerate_devices() {
        let dev = &mut crate::iter_devices().collect::<Vec<_>>()[0];
        let mut handle = dev.open().unwrap();
        let mut scope = handle.oscilloscope().unwrap();
        let mut channels = scope.channels().unwrap();
        dbg!(channels[0].range_steps());
        channels[0].offset_steps();
    }
    
}
