/// Returns the original string if not blank, otherwise the fallback.
///
/// A string is blank if it is empty or contains only whitespace.
///
/// # Examples
///
/// ```
/// use kotlinify::IfBlank;
///
/// assert_eq!("Kotlinify".if_blank(|| "Empty"), "Kotlinify");
/// assert_eq!("".if_blank(|| "Empty"), "Empty");
/// assert_eq!(" ".if_blank(|| "Empty"), "Empty");
/// ```
pub trait IfBlank {
    fn if_blank(self, fallback: impl FnOnce() -> Self) -> Self;
}

impl IfBlank for &str {
    fn if_blank(self, fallback: impl FnOnce() -> Self) -> Self {
        if self.trim().is_empty() {
            fallback()
        } else {
            self
        }
    }
}

impl IfBlank for String {
    fn if_blank(self, fallback: impl FnOnce() -> Self) -> Self {
        if self.trim().is_empty() {
            fallback()
        } else {
            self
        }
    }
}

/// Returns the contained value or its default value if `None`.
///
/// # Examples
///
/// ```
/// use kotlinify::OrEmpty;
///
/// let name: String = None.or_empty();
/// assert_eq!(name, "");
///
/// let numbers: Vec<i32> = None.or_empty();
/// assert!(numbers.is_empty());
///
/// let number: i32 = None.or_empty();
/// assert_eq!(number, 0)
/// ```
pub trait OrEmpty<T> {
    fn or_empty(self) -> T;
}

impl<T: Default> OrEmpty<T> for Option<T> {
    fn or_empty(self) -> T {
        self.unwrap_or_default()
    }
}
