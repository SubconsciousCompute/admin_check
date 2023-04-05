extern "C" {
    fn geteuid() -> std::ffi::c_uint;
    fn getuid() -> std::ffi::c_uint;
}

pub fn is_elevated() -> bool {
    // let _ = unsafe { getuid() };
    let effective = unsafe { geteuid() };

    // NOTE: might need tweaking on linux
    // `getuid` returns 0 when the program is run as admin on OS X.
    // but SO says `getuid` and `geteuid` should be different when the program is running as root
    if effective == 0 {
        true
    } else {
        false
    }
}
