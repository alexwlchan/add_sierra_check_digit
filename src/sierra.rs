use substring::Substring;

fn is_numeric(s: &str) -> bool {
    s.chars().all(|c| '0' <= c && c <= '9')
}

/// Adds the check digit to a Sierra record number.
///
/// This function takes the seven-digit record number and returns the
/// check digit that goes with this record number.
///
pub fn get_check_digit(record_number: &str) -> String {

  assert!(record_number.len() == 7);
  assert!(is_numeric(record_number));

  // Quoting from the Sierra manual:
  //
  //    Check digits may be any one of 11 possible digits (0, 1, 2, 3, 4,
  //    5, 6, 7, 8, 9, or x).
  //
  //    The check digit is calculated as follows:
  //
  //    Multiply the rightmost digit of the record number by 2, the next
  //    digit to the left by 3, the next by 4, etc., and total the products.
  //
  //    Divide the total by 11 and retain the remainder.  The remainder
  //    after the division is the check digit.  If the remainder is 10,
  //    the letter x is used as the check digit.
  //
  // See https://documentation.iii.com/sierrahelp/Default.htm#sril/sril_records_numbers.html
  //

  let chars_in_reverse =
    record_number
      .chars()
      .map(|c| c.to_digit(10).unwrap())
      .rev();

  let multiply_by = vec!(2, 3, 4, 5, 6, 7, 8);

  let total: u32 =
    chars_in_reverse.zip(multiply_by)
      .map(|(c, m)| c * m)
      .sum();

  let remainder = total % 11;

  if remainder == 10 {
    String::from("x")
  } else {
    remainder.to_string()
  }
}

/// Returns true if a given string is a valid Sierra record number, false otherwise.
///
/// The string can have an optional record type prefix.
///
pub fn validate(record_number: &str) -> bool {

    // Case #1: The string is nine chars long = record type prefix + record number
    //
    // Remove the record type prefix and validate the remainder.
    if record_number.len() == 9 {
        validate(record_number.substring(1, 10))
    }

    // Case #2: The string is eight chars long = seven digits + check digit
    //
    // The string is valid if and only if the string is numeric and the final digit
    // matches the check digit.
    else if record_number.len() == 8 {

        let number = record_number.substring(0, 7);
        let check_digit = record_number.substring(7, 8);

        is_numeric(number) && get_check_digit(number) == check_digit
    }

    // Case #3: any other string isn't a valid check digit.
    else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::sierra::{get_check_digit, validate};

    #[test]
    fn test_get_check_digit() {
        // This is an example from the Sierra documentation
        assert_eq!(get_check_digit("1024364"), "1");

        // Test the case where the remainder is an 'x'
        assert_eq!(get_check_digit("1026579"), "x");
    }

    #[test]
    fn test_validate() {
        assert_eq!(validate("10243641"), true);
        assert_eq!(validate("10243642"), false);

        // With a record type prefix
        assert_eq!(validate("b10243641"), true);
        assert_eq!(validate("b10243642"), false);
        assert_eq!(validate("i10243641"), true);
        assert_eq!(validate("i10243642"), false);

        assert_eq!(validate("short"), false);
        assert_eq!(validate("a too long string"), false);
        assert_eq!(validate("nonumber"), false);
    }
}
