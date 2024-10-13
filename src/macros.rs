#[macro_export]
macro_rules! qrversion {
    ($name:ident, $version:expr) => {
        const $name: usize = (4 * $version) + 17;
    };
}
