/// Returns the original collection if it is not empty, otherwise the fallback.
///
/// #Examples
///
/// ```
/// use kotlinify::IfEmpty;
///
/// let values = vec![1, 2, 3].if_empty(|| vec![0]);
/// assert_eq!(values, vec![1, 2, 3]);
///
/// let values = Vec::<i32>::new().if_empty(|| vec![0]);
/// assert_eq!(values, vec![0]);
/// ```
pub trait IfEmpty: Sized {
    /// Returns the original collection if it is not empty, otherwise the fallback.
    fn if_empty(self, fallback: impl FnOnce() -> Self) -> Self;
}

impl<T> IfEmpty for Vec<T> {
    fn if_empty(self, fallback: impl FnOnce() -> Self) -> Self {
        if self.is_empty() { fallback() } else { self }
    }
}
