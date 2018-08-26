use std::io;

pub fn add_one<T: io::Write>(digits: &[u8], output: &mut T) -> Result<(), io::Error> {
    // Parse the leading '-' for negative numbers.
    let (minus, digits) = match digits.split_first() {
        Some((&b'-', digits)) => (true, digits), // Negative number
        _ => (false, digits),
    };

    // Validate (ASCII) digits.
    if digits.is_empty() || !digits.iter().all(|&c| c >= b'0' && c <= b'9') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput.into(),
            "Invalid characters in input".to_string()
        ));
    }

    // Remove any leading zeros.
    let digits = &digits[digits.iter().position(|&c| c != b'0').unwrap_or(digits.len())..];

    // Find any trailing 9's (when positive) or 0's (when negative) which will carry the carry bit.
    let (prefix, trailing) = digits.split_at(
        digits.iter()
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
    } else {
        output.write_all(b"1")?;
    }
    for _ in 0..trailing.len() {
        output.write_all(if minus { b"9" } else { b"0" })?;
    }
    Ok(())
}

#[test]
fn add_one_test() {
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
        "1256146513513224524524524524522452165841613615616516517"
    );
    test(
        "1237801293471034709342345050491203491230949139249123949999999",
        "1237801293471034709342345050491203491230949139249123950000000"
    );
    test(
        "-1237801293471034709342345050491203491230949139249123940000000",
        "-1237801293471034709342345050491203491230949139249123939999999"
    );
}
