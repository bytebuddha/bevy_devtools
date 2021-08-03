macro_rules! diagnostic_value {
    ($diagnostics:ident, $item:expr) => {
        diagnostic_value!($diagnostics, $item, 0.0);
    };
    ($diagnostics:ident, $item:expr, $default:expr) => {
        $diagnostics
            .get($item)
            .expect(&format!("Failed to get Diagnostic"))
            .value()
            .unwrap_or($default)
    };
}
