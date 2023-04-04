use std::ffi::c_void;
use std::mem;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Security::{
    GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_ELEVATION_TYPE, TOKEN_QUERY,
};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

/// Returns a boolean value, indicating whether the current process is elevated.
/// ## Example
/// ```rust
/// use admin_check::is_elevated;
///
/// if !is_elevated() {
///     println!(
///         "Warning: the program isn’t running as elevated; some functionality may not work."
///     );
/// }
/// ```
pub fn is_elevated() -> bool {
    // based on https://stackoverflow.com/a/8196291
    unsafe {
        let mut current_token_ptr: HANDLE = mem::zeroed();
        let mut token_elevation: TOKEN_ELEVATION = mem::zeroed();
        let token_elevation_type_ptr: *mut TOKEN_ELEVATION = &mut token_elevation;
        let mut size = 0;

        let result = OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut current_token_ptr).0;

        if result != 0 {
            let result = GetTokenInformation(
                current_token_ptr,
                TokenElevation,
                Some(token_elevation_type_ptr as *mut c_void),
                mem::size_of::<TOKEN_ELEVATION_TYPE>() as u32,
                &mut size,
            )
                .0;
            if result != 0 {
                return token_elevation.TokenIsElevated != 0;
            }
        }
    }
    false
}
