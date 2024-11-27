#[allow(clippy::macro_metavars_in_unsafe)]

fn cstr_to_str(ptr: *const std::os::raw::c_char) -> Result<&'static str, std::str::Utf8Error> {
    // Unsafe block is isolated in a function
    unsafe { std::ffi::CStr::from_ptr(ptr).to_str() }
}

#[macro_export]
macro_rules! char_to_str {
    ($ptr:expr, $name:literal) => {
        match $crate::cstr_to_str($ptr) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to load {} {}", stringify!($name), e);
                return -1;
            }
        }
    };
}

#[macro_export]
macro_rules! jstring_to_str {
    ($ptr:expr, $env:ident, $name:literal) => {
        match $env.get_string($ptr) {
            Ok(s) => s.into(),
            Err(e) => {
                eprintln!("Failed to load {} {}", stringify!($name), e);
                return -1;
            }
        }
    };
}
