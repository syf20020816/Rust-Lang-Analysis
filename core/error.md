# Error

请参看``

- package : `error.rs`

## Preface



<hr />

## Ⓜ️Source

```rust
#[cfg(test)]
mod tests;

use crate::any::{Demand, Provider, TypeId};
use crate::fmt::{Debug, Display};

pub trait Error: Debug + Display {
    #[stable(feature = "error_source", since = "1.30.0")]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    /// Gets the `TypeId` of `self`.
    #[doc(hidden)]
    #[unstable(
        feature = "error_type_id",
        reason = "this is memory-unsafe to override in user code",
        issue = "60784"
    )]
    fn type_id(&self, _: private::Internal) -> TypeId
    where
        Self: 'static,
    {
        TypeId::of::<Self>()
    }

    #[unstable(feature = "error_generic_member_access", issue = "99301")]
    #[allow(unused_variables)]
    fn provide<'a>(&'a self, demand: &mut Demand<'a>) {}
}

#[unstable(feature = "error_generic_member_access", issue = "99301")]
impl<E> Provider for E
where
    E: Error + ?Sized,
{
    fn provide<'a>(&'a self, demand: &mut Demand<'a>) {
        self.provide(demand)
    }
}

mod private {
    #[unstable(feature = "error_type_id", issue = "60784")]
    #[derive(Debug)]
    pub struct Internal;
}

#[unstable(feature = "never_type", issue = "35121")]
impl Error for ! {}

impl<'a> dyn Error + 'a {
    /// Request a reference of type `T` as context about this error.
    #[unstable(feature = "error_generic_member_access", issue = "99301")]
    pub fn request_ref<T: ?Sized + 'static>(&'a self) -> Option<&'a T> {
        core::any::request_ref(self)
    }

    /// Request a value of type `T` as context about this error.
    #[unstable(feature = "error_generic_member_access", issue = "99301")]
    pub fn request_value<T: 'static>(&'a self) -> Option<T> {
        core::any::request_value(self)
    }
}

// Copied from `any.rs`.
impl dyn Error + 'static {
    /// Returns `true` if the inner type is the same as `T`.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn is<T: Error + 'static>(&self) -> bool {
        // Get `TypeId` of the type this function is instantiated with.
        let t = TypeId::of::<T>();

        // Get `TypeId` of the type in the trait object (`self`).
        let concrete = self.type_id(private::Internal);

        // Compare both `TypeId`s on equality.
        t == concrete
    }

    /// Returns some reference to the inner value if it is of type `T`, or
    /// `None` if it isn't.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn downcast_ref<T: Error + 'static>(&self) -> Option<&T> {
        if self.is::<T>() {
            // SAFETY: `is` ensures this type cast is correct
            unsafe { Some(&*(self as *const dyn Error as *const T)) }
        } else {
            None
        }
    }

    /// Returns some mutable reference to the inner value if it is of type `T`, or
    /// `None` if it isn't.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn downcast_mut<T: Error + 'static>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            // SAFETY: `is` ensures this type cast is correct
            unsafe { Some(&mut *(self as *mut dyn Error as *mut T)) }
        } else {
            None
        }
    }
}

impl dyn Error + 'static + Send {
    /// Forwards to the method defined on the type `dyn Error`.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn is<T: Error + 'static>(&self) -> bool {
        <dyn Error + 'static>::is::<T>(self)
    }

    /// Forwards to the method defined on the type `dyn Error`.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn downcast_ref<T: Error + 'static>(&self) -> Option<&T> {
        <dyn Error + 'static>::downcast_ref::<T>(self)
    }

    /// Forwards to the method defined on the type `dyn Error`.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn downcast_mut<T: Error + 'static>(&mut self) -> Option<&mut T> {
        <dyn Error + 'static>::downcast_mut::<T>(self)
    }

    /// Request a reference of type `T` as context about this error.
    #[unstable(feature = "error_generic_member_access", issue = "99301")]
    pub fn request_ref<T: ?Sized + 'static>(&self) -> Option<&T> {
        <dyn Error>::request_ref(self)
    }

    /// Request a value of type `T` as context about this error.
    #[unstable(feature = "error_generic_member_access", issue = "99301")]
    pub fn request_value<T: 'static>(&self) -> Option<T> {
        <dyn Error>::request_value(self)
    }
}

impl dyn Error + 'static + Send + Sync {
    /// Forwards to the method defined on the type `dyn Error`.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn is<T: Error + 'static>(&self) -> bool {
        <dyn Error + 'static>::is::<T>(self)
    }

    /// Forwards to the method defined on the type `dyn Error`.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn downcast_ref<T: Error + 'static>(&self) -> Option<&T> {
        <dyn Error + 'static>::downcast_ref::<T>(self)
    }

    /// Forwards to the method defined on the type `dyn Error`.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn downcast_mut<T: Error + 'static>(&mut self) -> Option<&mut T> {
        <dyn Error + 'static>::downcast_mut::<T>(self)
    }

    /// Request a reference of type `T` as context about this error.
    #[unstable(feature = "error_generic_member_access", issue = "99301")]
    pub fn request_ref<T: ?Sized + 'static>(&self) -> Option<&T> {
        <dyn Error>::request_ref(self)
    }

    /// Request a value of type `T` as context about this error.
    #[unstable(feature = "error_generic_member_access", issue = "99301")]
    pub fn request_value<T: 'static>(&self) -> Option<T> {
        <dyn Error>::request_value(self)
    }
}

impl dyn Error {
    pub fn sources(&self) -> Source<'_> {
        Source { current: Some(self) }
    }
}

#[unstable(feature = "error_iter", issue = "58520")]
#[derive(Clone, Debug)]
pub struct Source<'a> {
    current: Option<&'a (dyn Error + 'static)>,
}

#[unstable(feature = "error_iter", issue = "58520")]
impl<'a> Iterator for Source<'a> {
    type Item = &'a (dyn Error + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.current.and_then(Error::source);
        current
    }
}

#[stable(feature = "error_by_ref", since = "1.51.0")]
impl<'a, T: Error + ?Sized> Error for &'a T {
    #[allow(deprecated, deprecated_in_future)]
    fn description(&self) -> &str {
        Error::description(&**self)
    }

    #[allow(deprecated)]
    fn cause(&self) -> Option<&dyn Error> {
        Error::cause(&**self)
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Error::source(&**self)
    }

    fn provide<'b>(&'b self, demand: &mut Demand<'b>) {
        Error::provide(&**self, demand);
    }
}

#[stable(feature = "fmt_error", since = "1.11.0")]
impl Error for crate::fmt::Error {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "an error occurred when formatting an argument"
    }
}

#[stable(feature = "try_borrow", since = "1.13.0")]
impl Error for crate::cell::BorrowError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "already mutably borrowed"
    }
}

#[stable(feature = "try_borrow", since = "1.13.0")]
impl Error for crate::cell::BorrowMutError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "already borrowed"
    }
}

#[stable(feature = "try_from", since = "1.34.0")]
impl Error for crate::char::CharTryFromError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "converted integer out of range for `char`"
    }
}

#[stable(feature = "duration_checked_float", since = "1.66.0")]
impl Error for crate::time::TryFromFloatSecsError {}

#[stable(feature = "cstr_from_bytes_until_nul", since = "1.69.0")]
impl Error for crate::ffi::FromBytesUntilNulError {}

#[unstable(feature = "get_many_mut", issue = "104642")]
impl<const N: usize> Error for crate::slice::GetManyMutError<N> {}

```

## Code