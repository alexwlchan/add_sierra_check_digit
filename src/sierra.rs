//! Adds the check digit to a Sierra record number.
//!
//! This function takes the seven-digit record number and returns the
//! check digit that goes with this record number.
//!
pub fn get_check_digit(record_number: &str) -> String {

  assert!(record_number.len() == 7);
  assert!(record_number.chars().all(|c| '0' <= c && c <= '9'));

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

#[cfg(test)]
mod tests {
    use crate::sierra::get_check_digit;

    #[test]
    fn test_get_check_digit_example() {
        // This is an example from the Sierra documentation
        assert_eq!(get_check_digit("1024364"), "1");
    }
}
