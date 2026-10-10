/// Converts an integer to a Roman numeral.
///
/// Supports values from 1 to 3999.
///
/// # Examples
///
/// ```
/// use kotlinify::ToRoman;
///
/// assert_eq!(1_u32.to_roman().unwrap(), "I")
/// ```
pub trait ToRoman {
    fn to_roman(self) -> Result<String, InvalidRomanNumeral>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InvalidRomanNumeral;

impl std::fmt::Display for InvalidRomanNumeral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value must be between 1 and 3999")
    }
}

impl std::error::Error for InvalidRomanNumeral {}

impl ToRoman for u32 {
    fn to_roman(self) -> Result<String, InvalidRomanNumeral> {
        if !(1..=3999).contains(&self) {
            return Err(InvalidRomanNumeral);
        }

        let numerals = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut number = self;
        let mut result = String::new();

        for (value, numeral) in numerals {
            while number >= value {
                result.push_str(numeral);
                number -= value;
            }
        }

        Ok(result)
    }
}
