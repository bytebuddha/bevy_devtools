macro_rules! diagnostic_value {
    ($diagnostics:ident, $item:expr) => {
        diagnostic_value!($diagnostics, $item, 0.0)
    };
    ($diagnostics:ident, $item:expr, $default:expr) => {
        $diagnostics
            .get($item)
            .expect(&format!("Failed to get Diagnostic"))
            .value()
            .unwrap_or($default)
    };
}

macro_rules! ignore_none_error {
    ($expr:expr, $msg:tt) => {
        match $expr {
            Some(pos) => pos,
            None => {
                warn!($msg);
                return;
            }
        }
    };
}

macro_rules! ignore_error {
    ($expr:expr) => {
        match $expr {
            Ok(item) => item,
            Err(err) => {
                warn!("{}", err);
                return;
            }
        }
    };
}
