// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {

    let mut no_of_chars = 0;
    let mut sum = 0;
    for (x,y) in cc_number.chars().rev().filter(|&x| x != ' ' ).enumerate()
    {
        match y.to_digit(10) {
                Some(dig) => {
                    sum += if x % 2 == 1 {
                        let d2 = dig * 2;
                        d2/10 + d2 % 10
                    }
                    else {dig};
                    no_of_chars += 1;
                },
                None => return false,
        }
    }

    if no_of_chars < 2 {
        return false;
    }
    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}