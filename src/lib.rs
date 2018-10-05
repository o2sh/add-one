//! This add-one crate is meant to give you the opportunity to add one to a number.
//! 
//! # Examples
//! 
//! ```
//!extern crate add_one;
//!use add_one::add_one;
//!
//!use std::str;
//!
//!fn main() {
//!    let mut bytes = Vec::new();
//!
//!    match add_one("123".as_bytes(), &mut bytes) {
//!        Ok(()) => println!("{}", str::from_utf8(&bytes).unwrap()),
//!        Err(e) => {
//!            eprintln!("Error: {}", e);
//!        }
//!    }
//!}
//! ```
//!
//! ## Compatibility
//!
//! The `add-one` crate is tested for rustc 1.26 and greater.

use std::io;

pub fn add_one<T: io::Write>(digits: &[u8], output: &mut T) -> Result<(), io::Error> {
    // Parse the leading '-' for negative numbers.
    let (minus, digits) = match digits.split_first() {
        Some((&b'-', digits)) => (true, digits), // Negative number
        _ => (false, digits),
    };

    // Validate (ASCII) digits.
    if digits.is_empty() || !digits.iter().all(|&c| c >= b'0' && c <= b'9' || c == b'.') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid characters in input".to_string(),
        ));
    }

    // Remove any leading zeros.
    let digits = &digits[digits.iter().position(|&c| c != b'0').unwrap_or(digits.len())..];

    let z = digits.iter().position(|&c| c == b'.').unwrap_or(digits.len()); // position of decimal

    // Find any trailing 9's (when positive) or 0's (when negative) which will carry the carry bit.
    let (prefix, trailing) = digits[..z].split_at(
        digits[..z].iter()
            .rposition(|&c| c != if minus { b'0' } else { b'9' })
            .map_or(0, |x| x + 1) // The position *after* the last non-nine/zero.
    );

    if let Some((&digit, prefix)) = prefix.split_last() {
        // The last digit before that will have to be incremented/decremented,
        // and anything before it is left unchanged.
        let new_digit = if minus {
            if prefix.is_empty() && trailing.is_empty() && digit == b'1' {
                // Special case for -1: Result is no longer negative.
                return output.write_all(b"0");
            }
            output.write_all(b"-")?;
            digit - 1
        } else {
            digit + 1
        };
        // Write prefix, unchanged.
        output.write_all(prefix)?;
        // Write changed digit, unless it became a leading zero.
        if !prefix.is_empty() || new_digit != b'0' {
            output.write_all(&[new_digit])?;
        }
    } else if minus && digits.len() > 0 && digits[0] == b'.' && !digits[1..].iter().all(|&d| d == b'0') {
        output.write_all(b"0")?;
    } else {
        output.write_all(b"1")?;
    }
    for _ in 0..trailing.len() {
        output.write_all(if minus { b"9" } else { b"0" })?;
    }
    if minus && z == 0 && digits.len() > 1 {
        // write decimal
        output.write_all(&digits[0..1])?;
        let mut iter = digits[1..].iter().rev().peekable();
        let mut decimal_digits = Vec::new();
        while let Some(&b'0') = iter.peek() {
            decimal_digits.push(*iter.next().unwrap());
        }
        if let Some(_) = iter.peek() {
            decimal_digits.push(b'9' - *iter.next().unwrap() + b'0' + 1);
        }
        while let Some(_) = iter.peek() {
            decimal_digits.push(b'9' - *iter.next().unwrap() + b'0');
        }
        output.write_all(decimal_digits.iter().rev().cloned().collect::<Vec<u8>>().as_slice())?;
    } else {
        output.write_all(&digits[z..])?;// prints the characters after decimal
    }
    Ok(())
}

#[test]
fn add_one_test_integer() {
    fn test(num: &str, result: &str) {
        use std::str::from_utf8;
        let mut s = Vec::new();
        add_one(num.as_bytes(), &mut s).unwrap();
        assert_eq!(from_utf8(&s).unwrap(), result);
    }
    test("1234", "1235");
    test("0", "1");
    test("9", "10");
    test("19", "20");
    test("99", "100");
    test("99988999", "99989000");
    test("99999999", "100000000");
    test("0001234", "1235");
    test("-9", "-8");
    test("-10", "-9");
    test("-100", "-99");
    test("-0100", "-99");
    test("-1100", "-1099");
    test("-01100", "-1099");
    test("-1", "0");
    test("-001", "0");
    test("-0", "1");
    test("-000", "1");
    test(
        "1256146513513224524524524524522452165841613615616516516",
        "1256146513513224524524524524522452165841613615616516517",
    );
    test(
        "1237801293471034709342345050491203491230949139249123949999999",
        "1237801293471034709342345050491203491230949139249123950000000",
    );
    test(
        "-1237801293471034709342345050491203491230949139249123940000000",
        "-1237801293471034709342345050491203491230949139249123939999999",
    );
    test("-2", "-1");
}

#[test]
fn add_one_test_float() {
    fn test(num: &str, result: &str) {
        use std::str::from_utf8;
        let mut s = Vec::new();
        add_one(num.as_bytes(), &mut s).unwrap();
        assert_eq!(from_utf8(&s).unwrap(), result);
    }
    test("0.0", "1.0");
    test("5000.0", "5001.0");
    test("1139.67", "1140.67");
    test("123.321", "124.321");
    test("99.99", "100.99");
    test("-1.0", "0");
    test("2.0", "3.0");
    test("000.000", "1.000");
    test("-000.00", "1.00");
    test("-0.9", "0.1");
    test("0123456789.0987654321", "123456790.0987654321");
}

