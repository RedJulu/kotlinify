/// Returns the value if it matches a predicate.
///
/// Returns `Some(value)` if the predicate returns `true`, otherwise `None`.
///
/// The predicate receives a reference to the value.
///
/// # Examples
///
/// ```
/// use kotlinify::TakeIf;
///
/// let value = 42;
/// let result = value.take_if(|it| *it > 10);
///
/// assert_eq!(result, Some(42));
/// assert_eq!(5.take_if(|it| *it > 10), None);
/// ```
pub trait TakeIf: Sized {
    /// Returns the value if the predicate returns `true`
    fn take_if(self, predicate: impl FnOnce(&Self) -> bool) -> Option<Self>;
}

impl<T> TakeIf for T {
    fn take_if(self, predicate: impl FnOnce(&Self) -> bool) -> Option<Self> {
        if predicate(&self) { Some(self) } else { None }
    }
}

/// Returns the value if it doesn't match a predicate.
///
/// Returns `Some(value)` if the predicate returns `false`, otherwise `None`.
///
/// The predicate receives a reference to the value.
///
/// # Examples
///
/// ```
/// use kotlinify::TakeUnless;
///
/// let value = 42;
/// let result = value.take_unless(|it| *it < 10);
///
/// assert_eq!(result, Some(42));
/// assert_eq!(5.take_unless(|it| *it < 10), None);
/// ```
pub trait TakeUnless: Sized {
    /// Returns the value if the predicate returns `false`
    fn take_unless(self, predicate: impl FnOnce(&Self) -> bool) -> Option<Self>;
}

impl<T> TakeUnless for T {
    fn take_unless(self, predicate: impl FnOnce(&Self) -> bool) -> Option<Self> {
        if predicate(&self) { None } else { Some(self) }
    }
}
