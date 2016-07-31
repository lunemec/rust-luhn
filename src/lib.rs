extern crate num;
extern crate num_digitize;

use num::*;
use num_digitize::ToDigits;


/// Takes a borrow of vector containing digits of credit card `&Vec<u8>` and a function
/// `take_function(&usize) -> bool`. This function is called for each item of digit vector and if
/// `take_function(digit) == true`, then it is inserted onto index 0 of resulting vector.
/// By this action, the resulting vector is reversed.
fn take_digits<F>(card_number_digits: &Vec<i8>, mut take_function: F) -> Vec<i8>
    where F: FnMut(&usize) -> bool
{
    let mut result: Vec<i8> = Vec::new();
    for item in card_number_digits.iter().enumerate() {
        if take_function(&item.0) {
            result.insert(0, *item.1);
        }
    }
    result
}

/// Since we can't properly use `.sum()` on `Iterator` because of the
/// 'iter_arith': bounds recently changed (see issue #27739), we will sum manually.
fn sum(vector: &Vec<i8>) -> u64 {
    let mut result: u64 = 0;
    for item in vector {
        result += *item as u64;
    }
    result
}


/// Luhn algorithm for credit card number validation (may be used for other purposes).
///
/// # Arguments
///
/// * `number` - number to validate.
///
/// # Example
///
/// ```
/// use luhn2::validate;
///
/// let number: u64 = 49927398716;
/// assert!(validate(number));
/// ```
pub fn validate(number: u64) -> bool {
    let vec = {
        let mut vec = number.to_digits();
        vec.reverse();
        vec
    };

    // Sum odd indexed digits. We pass `is_even()` function because we index from 0, not from 1.
    let odd_sum: u64 = sum(&take_digits(&vec, Integer::is_even));
    let even_sum: u64 = {
        let mut even_sum: u64 = 0;
        // Take even indexed digits (indexed from 0).
        let even_digits = take_digits(&vec, Integer::is_odd);
        // Multiply each even indexed number by 2.
        for even_digit in even_digits.iter().map(|x| x * 2) {
            // Convert each number to digits and sum them.
            even_sum += sum(&even_digit.to_digits());
        }
        even_sum
    };

    let luhn = odd_sum + even_sum;
    return luhn % 10 == 0;
}

#[test]
fn test_sum() {
    assert!(sum(&vec![1, 2, 3, 4]) == 10);
    assert!(sum(&vec![]) == 0);
}

#[test]
fn test_take_digits() {
    let test_digits = vec![0, 1, 2, 3, 4, 5, 6];
    assert!(take_digits(&test_digits, Integer::is_even) == vec![6, 4, 2, 0]);
    assert!(take_digits(&test_digits, Integer::is_odd) == vec![5, 3, 1]);
}

#[test]
fn test_ok_numbers() {
    assert!(validate(49927398716));
    assert!(validate(1234567812345670));
    assert!(validate(79927398713));
}

#[test]
fn test_invalid_numbers() {
    assert!(validate(4242424242424241) == false);
    assert!(validate(49927398717) == false);
    assert!(validate(1234567812345678) == false);
}
