#[allow(clippy::macro_metavars_in_unsafe)]
#[macro_export]
macro_rules! char_to_str {
    ($ptr:expr, $name:literal) => {
        match unsafe { std::ffi::CStr::from_ptr($ptr) }.to_str() {
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
