// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{clamp, concat, div};

    #[test]
    fn clamp_below_range() {
        let result = clamp(1, 10, 20);
        let expected = 10;
        assert_eq!(result, expected, "Should take the 'lower' value");
    }
    #[test]
    fn clamp_above_range() {
        let result = clamp(25, 10, 20);
        let expected = 20;
        assert_eq!(result, expected, "Should take the 'upper' value");
    }
    #[test]
    fn clamp_within_range() {
        let result = clamp(15, 10, 20);
        let expected = 15;
        assert_eq!(result, expected, "Should take the 'n' value");
    }

    #[test]
    fn div_by_numbers() {
        let result = div(10, 2);
        let expected = Some(5);
        assert_eq!(result, expected, "Should divide the numbers");
    }
    #[test]
    fn div_by_zero() {
        let result = div(10, 0);
        let expected = None;
        assert_eq!(result, expected, "Should return None when dividing by zero");
    }
    #[test]
    fn div_zero_by_zero() {
        let result = div(0, 10);
        let expected = Some(0);
        assert_eq!(
            result, expected,
            "Should return 0 when dividing zero by a number"
        );
    }

    #[test]
    fn concat_strings() {
        let result = concat("a", "b");
        let expected = "a b";
        assert_eq!(result, expected, "Should concat strings");

        let result = concat("", "b");
        let expected = " b";
        assert_eq!(result, expected, "Should concat an empty string");
    }
    #[test]
    fn concat_empty_string() {
        let result = concat("", "b");
        let expected = " b";
        assert_eq!(result, expected, "Should concat an empty string");
    }
}
