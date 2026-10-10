#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidBounds;

/// Clamps a value to a minimum.
///
/// # Examples
///
/// ```
/// use kotlinify::CoerceAtLeast;
///
/// let (value, changed) = 5.coerce_at_least(10);
///
/// assert_eq!(value, 10);
/// assert!(changed);
///
/// ```
pub trait CoerceAtLeast: Sized {
    /// Ensures that the value is at least `minimum`.
    fn coerce_at_least(self, minimum: Self) -> (Self, bool);
}

/// Clamps a value to a maximum.
///
/// # Examples
///
/// ```
/// use kotlinify::CoerceAtMost;
///
/// let (value, changed) = 15.coerce_at_most(10);
///
/// assert_eq!(value, 10);
/// assert!(changed);
///
/// ```
pub trait CoerceAtMost: Sized {
    /// Ensures that the value is at most `maximum`.
    fn coerce_at_most(self, maximum: Self) -> (Self, bool);
}

/// Clamps a value to an inclusive range.
///
/// Returns an error if `minimum` is greater than `maximum`.
///
/// # Examples
///
/// ```
/// use kotlinify::CoerceIn;
///
/// let (value, changed) = 5.coerce_in(10, 20).unwrap();
///
/// assert_eq!(value, 10);
/// assert!(changed);
/// ```
pub trait CoerceIn: Sized {
    /// Ensures that the value is between `minimum` and `maximum`.
    fn coerce_in(self, minimum: Self, maximum: Self) -> Result<(Self, bool), InvalidBounds>;
}

macro_rules! impl_coerce {
    ($($type:ty),*) => {
        $(
            impl CoerceAtLeast for $type {
                fn coerce_at_least(self, minimum: Self) -> (Self, bool) {
                    if self < minimum {
                        (minimum, true)
                    } else {
                        (self, false)
                    }
                }
            }

            impl CoerceAtMost for $type {
                fn coerce_at_most(self, maximum: Self) -> (Self, bool) {
                    if self > maximum {
                        (maximum, true)
                    } else {
                        (self, false)
                    }
                }
            }

            impl CoerceIn for $type {
                fn coerce_in(self, minimum: Self, maximum: Self) -> Result<(Self, bool), InvalidBounds> {
                    if minimum > maximum {
                        return Err(InvalidBounds);
                    }

                    if self < minimum {
                        Ok((minimum, true))
                    } else if self > maximum {
                        Ok((maximum, true))
                    } else {
                        Ok((self, false))
                    }
                }
            }
        )*
    };
}

impl_coerce!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);
