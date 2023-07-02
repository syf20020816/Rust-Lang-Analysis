//! 实现一元运算符“op&T”
//!  基于“操作T”，其中T应为“可复制”
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/2
//! @version:0.0.1
//! @description:
//! ```

/// you may see the source:
/// ``` code
/// (impl const $imp:ident, $method:ident for $t:ty, $u:ty) => {
///         forward_ref_binop!(impl const $imp, $method for $t, $u,
///                 #[stable(feature = "rust1", since = "1.0.0")]);
///     };
/// ```
/// but actually! `#[stable(feature = "rust1", since = "1.0.0")]);` is no useful to us
// #![feature(const_trait_impl)]
#[macro_export]
macro_rules! forward_ref_binop {
    (impl const $impl:ident, $method:ident for $t:ty,$u:ty) => {
        /// force:`forward_ref_binop!{impl const Add, add for $t, $t }`
        /// <hr>
        /// so we know : $t == $u

        /// such as:
        /// ``` code
        /// impl<'a> const Add<u8> for &'a u8{
        ///   // just like Self::Output
        ///   type Output = <u8 as Add<u8>>::Output;
        ///   fn add(self, rhs: u8)-><u8 as Add<u8>>::Output{
        ///       Add::add(*self,rhs)
        ///   }
        /// }
        /// ```
        impl<'a> const $impl<$u> for &'a $t {
            type Output = <$t as $impl<$u>>::Output;

            fn $method(self, rhs: $u)-><$t as $impl<$u>>::Output{
                $impl::$method(*self, rhs)
            }
        }
        // such as:
        // impl const $imp<&u8> for u8 {}
        impl const $impl<&$u> for $t{
            type Output = <$t as $impl<$u>>::Output;

            fn $method(self, rhs: &$u)-><$t as $impl<$u>>::Output{
                $impl::$method(self, *rhs)
            }
        }
        // such as:
        // impl const $imp<&u8> for &u8{}
        impl const $impl<&$u> for &$t {
            type Output = <$t as $impl<$u>>::Output;

            fn $method(self, rhs: &$u)-><$t as $impl<$u>>::Output{
                $impl::$method(*self, *rhs)
            }
        }
    };
    (impl $impl:ident, $method:ident for $t:ty,$u:ty) => {
        impl<'a>  $impl<$u> for &'a $t {
            type Output = <$t as $impl<$u>>::Output;

            fn $method(self, rhs: $u)-><$t as $impl<$u>>::Output{
                $impl::$method(*self, rhs)
            }
        }
        impl $impl<&$u> for $t{
            type Output = <$t as $impl<$u>>::Output;

            fn $method(self, rhs: &$u)-><$t as $impl<$u>>::Output{
                $impl::$method(self, *rhs)
            }
        }
        impl $imp<&$u> for &$t {
            type Output = <$t as $impl<$u>>::Output;

            fn $method(self, rhs: &$u)-><$t as $impl<$u>>::Output{
                $impl::$method(*self, *rhs)
            }
        }
    };
}