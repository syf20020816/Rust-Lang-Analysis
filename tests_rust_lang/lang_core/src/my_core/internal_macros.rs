/// you may see the source:
/// ``` code
/// (impl const $imp:ident, $method:ident for $t:ty, $u:ty) => {
///         forward_ref_binop!(impl const $imp, $method for $t, $u,
///                 #[stable(feature = "rust1", since = "1.0.0")]);
///     };
/// ```
/// but actually! `#[stable(feature = "rust1", since = "1.0.0")]);` is no useful to us
#[macro_export]
macro_rules! forward_ref_binop {
    (impl const $impl:ident, $method:ident for $t:ty,$u:ty) => {
        // such as:
        // impl<'a> const Add<> for &'a u8{}
        impl<'a> const $impl<$u> for &'a $t {

        }
        // such as:
        // impl const $imp<&u8> for $t {}

        // such as:
        // impl const $imp<&$u> for &$t
    };
}