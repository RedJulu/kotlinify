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
