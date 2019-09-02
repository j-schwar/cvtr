use std::error::Error;
use std::fmt::{self, Display};

/// Error type for radix conversions.
#[derive(Debug, PartialEq)]
pub enum RadixError {
    UnableToParse(String, u32),
    UnableToFormat(u64, u32),
}

impl Error for RadixError {}

impl Display for RadixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            RadixError::UnableToParse(s, radix) => {
                write!(f, "unable to parse '{}' with radix {}", s, radix)
            }
            RadixError::UnableToFormat(n, radix) => {
                write!(f, "unable to format {}, with radix {}", n, radix)
            }
        }
    }
}

/// Splits a numeral string on it's radix prefix.
///
/// # Examples
///
/// ```
/// use cvtr::radix;
///
/// assert_eq!(radix::strip_prefix("0xa"), ("0x", "a"));
/// ```
pub fn strip_prefix(s: &str) -> (&str, &str) {
    if s.len() > 2 && &s[0..2] == "0x" {
        (&s[0..2], &s[2..])
    } else if s.len() > 2 && &s[0..2] == "0b" {
        (&s[0..2], &s[2..])
    } else if s.len() > 1 && &s[0..1] == "0" {
        (&s[0..1], &s[1..])
    } else {
        ("", s)
    }
}

/// Returns the radix for a given numeral `prefix`. In the case of an invalid
/// prefix, `None` is returned.
///
/// # Examples
///
/// ```
/// use cvtr::radix;
///
/// assert_eq!(radix::detect("0x"), Some(16));
/// ```
///
/// An empty `prefix` is interpreted as a radix of `10`.
///
/// ```
/// use cvtr::radix;
///
/// assert_eq!(radix::detect(""), Some(10));
/// ```
pub fn detect(prefix: &str) -> Option<u32> {
    match prefix {
        "0b" => Some(2),
        "0" => Some(8),
        "" => Some(10),
        "0x" => Some(16),
        _ => None,
    }
}

/// Converts `n` to a string using a given `radix`.
///
/// # Errors
///
/// Returns an `Err` if attempting to format using an unsupported `radix`.
/// Currently only 2, 5, 10, and 16 are supported as values for `radix`.
///
/// # Examples
///
/// ```
/// use cvtr::radix;
///
/// assert_eq!(radix::format(18, 16), Ok(String::from("12")));
/// ```
pub fn format(n: u64, radix: u32) -> Result<String, RadixError> {
    match radix {
        2 => Ok(format!("{:b}", n)),
        8 => Ok(format!("{:o}", n)),
        10 => Ok(format!("{}", n)),
        16 => Ok(format!("{:x}", n)),
        _ => Err(RadixError::UnableToFormat(n, radix)),
    }
}

/// Converts a numeral string, `s`, from one radix to another.
///
/// # Parameters
///
/// * `s`: The string to convert. Should be a numeral string **without** a prefix.
/// * `from_radix`: The current radix of `s`.
/// * `to_radix`: The radix to convert `s` to.
///
/// # Errors
///
/// Returns `Err` if unable to parse `s` using `from_radix` as a base. This could
/// be because `s` is not a numeral at all, or it is not a valid numeral in the
/// base `from_radix`.
///
/// # Examples
///
/// ```
/// use cvtr::radix;
///
/// assert_eq!(radix::convert("a", 16, 10), Ok(String::from("10")));
/// ```
pub fn convert(s: &str, from_radix: u32, to_radix: u32) -> Result<String, RadixError> {
    let n = u64::from_str_radix(s, from_radix)
        .map_err(|_| RadixError::UnableToParse(s.to_string(), from_radix))?;
    format(n, to_radix)
}

/// Returns a string representation of a given radix. For example, "hex" for
/// radix-16.
pub fn as_text(radix: u32) -> String {
    match radix {
        2 => String::from("binary"),
        8 => String::from("octal"),
        10 => String::from("decimal"),
        16 => String::from("hex"),
        n => format!("radix-{}", n),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn strip_prefix_empty_arg() {
        assert_eq!(strip_prefix(""), ("", ""));
    }

    #[test]
    fn strip_prefix_expected() {
        assert_eq!(strip_prefix("0b1101"), ("0b", "1101"));
        assert_eq!(strip_prefix("074"), ("0", "74"));
        assert_eq!(strip_prefix("1045"), ("", "1045"));
        assert_eq!(strip_prefix("0xaf9"), ("0x", "af9"));
    }

    #[test]
    fn detect_expected() {
        assert_eq!(detect("0b"), Some(2));
        assert_eq!(detect("0"), Some(8));
        assert_eq!(detect(""), Some(10));
        assert_eq!(detect("0x"), Some(16));
        assert_eq!(detect("hello"), None);
    }

    #[test]
    fn format_err() {
        assert!(format(10, 7).is_err());
    }

    #[test]
    fn format_expected() {
        assert_eq!(format(10, 2), Ok(String::from("1010")));
        assert_eq!(format(10, 8), Ok(String::from("12")));
        assert_eq!(format(10, 10), Ok(String::from("10")));
        assert_eq!(format(10, 16), Ok(String::from("a")));
    }

    #[test]
    fn convert_expected() {
        assert_eq!(convert("1010", 2, 10), Ok(String::from("10")));
        assert_eq!(convert("10", 8, 10), Ok(String::from("8")));
        assert_eq!(convert("10", 10, 10), Ok(String::from("10")));
        assert_eq!(convert("10", 16, 10), Ok(String::from("16")));
        assert_eq!(convert("10", 10, 2), Ok(String::from("1010")));
        assert_eq!(convert("10", 10, 8), Ok(String::from("12")));
        assert_eq!(convert("10", 10, 16), Ok(String::from("a")));
    }
}
