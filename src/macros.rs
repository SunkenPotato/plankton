#[macro_export]
/// Specify a value to use in debug and in release.
///
/// Example:
/// ```rust
/// const A: i32 = debug_value!(1, 2); // in debug, this is `1`, in release `2`
/// ```
macro_rules! debug_value {
    ($db:expr, $rel:expr) => {
        '__dv: {
            #[cfg(debug_assertions)]
            break '__dv $db;
            #[cfg(not(debug_assertions))]
            break '__dv $rel;
        }
    };
}
