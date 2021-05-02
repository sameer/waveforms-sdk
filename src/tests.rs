#[test]
fn version() {
    dbg!(crate::version());
}

#[test]
fn enumerate_devices() {
    dbg!(crate::enumerate_devices().collect::<Vec<_>>());
}
