#[macro_export]
macro_rules! extract_param {
    ($data:expr, $field:ident, $default:expr) => {
        if let Some(inner_data) = $data.clone() {
            inner_data.$field
        } else {
            $default
        }
    };
}
