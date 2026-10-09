/// Executes an action with a refrence to the value and returns the value.
///
/// # Examples
///
/// ```
/// use kotlinify::Also;
///
/// let value = 42.also(|it| println!("{it}"));
///
/// assert_eq!(value, 42);
/// ```
pub trait Also: Sized {
    /// Executes an action with a reference to the value.
    fn also(self, action: impl FnOnce(&Self)) -> Self;
}

impl<T> Also for T {
    fn also(self, action: impl FnOnce(&Self)) -> Self {
        action(&self);
        self
    }
}

/// Executes a block with the contained value if it is `Some`.
///
/// Returns `Some(result)` if the option contains a value, otherwise `None`.
///
/// # Examples
///
/// ```
/// use kotlinify::Let;
///
/// let name = Some("Kotlinify");
/// let lenght = name.klet(|it| it.len());
///
/// assert_eq!(lenght, Some(9));
///
/// let name: Option<&str> = None;
/// let lenght = name.klet(|it| it.len());
///
/// assert_eq!(lenght, None);
/// ```
pub trait Let<T> {
    /// Transforms the contained value if it is `Some`.
    fn klet<R>(self, transform: impl FnOnce(T) -> R) -> Option<R>;
}

impl<T> Let<T> for Option<T> {
    fn klet<R>(self, transform: impl FnOnce(T) -> R) -> Option<R> {
        self.map(transform)
    }
}

/// Executes a block with ownership of the value and returns the block's result.
///
/// # Examples
///
/// ```
/// use kotlinify::Run;
///
/// let length = String::from("Kotlinify").run(|it| it.len());
///
/// assert_eq!(length, 9)
/// ```
pub trait Run: Sized {
    /// Executes a block with ownership of the value.
    fn run<R>(self, block: impl FnOnce(Self) -> R) -> R;
}

impl<T> Run for T {
    fn run<R>(self, block: impl FnOnce(Self) -> R) -> R {
        block(self)
    }
}
