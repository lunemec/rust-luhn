extern crate num;
extern crate num_digitize;

use num::*;
use num_digitize::digitize;

struct CreditCard {
    number: u64,
    csc: u16, // https://en.wikipedia.org/wiki/Card_security_code
}

trait Validate {
    fn validate(&self) -> bool;
}

impl Validate for CreditCard {
    fn validate(&self) -> bool {
        return validate_card_number(self.number);
    }
}


/// TODO
fn take_digits<F>(card_number_digits: &Vec<u8>, mut take_function: F) -> Vec<u8>
    where F: FnMut(&usize) -> bool
{
    let mut result: Vec<u8> = Vec::new();
    for item in card_number_digits.iter().enumerate() {
        if take_function(&item.0) {
            result.insert(0, *item.1);
        }
    }
    result
}

/// Since we can't properly use `.sum()` on `Iterator` because of the
/// 'iter_arith': bounds recently changed (see issue #27739), we will sum manually.
fn sum(vector: &Vec<u8>) -> u64 {
    let mut result: u64 = 0;
    for item in vector {
        result += *item as u64;
    }
    result
}


/// TODO
pub fn validate_card_number(card_number: u64) -> bool {
    let vec = {
        let mut vec = digitize(card_number);
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
            even_sum += sum(&digitize(even_digit));
        }
        even_sum
    };

    let luhn = odd_sum + even_sum;
    return luhn % 10 == 0;
}

#[test]
fn test_ok_numbers() {
    assert!(validate_card_number(49927398716));
    assert!(validate_card_number(1234567812345670));
    assert!(validate_card_number(79927398713));
}

#[test]
fn test_invalid_numbers() {
    assert!(validate_card_number(4242424242424241) == false);
    assert!(validate_card_number(49927398717) == false);
    assert!(validate_card_number(1234567812345678) == false);
}
