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

/// Given a vector containing individual digits of credit card number, return
/// a vector containing only digits on even indexes.
/// ```
/// let card_number_digits: Vec<u8> = vec![1,2,3,4];
/// println!("{:?}", even_digits(card_number_digits)));
/// [2,4]
/// ```
fn even_digits(card_number_digits: &Vec<u8>) -> Vec<u8> {
    let even_iter = card_number_digits.iter().enumerate().filter(|&x| x.0.is_odd());
    let mut digits: Vec<u8> = Vec::new();
    for x in even_iter {
        digits.push(*x.1);
    }

    return digits;
}

fn two_times_each_and_sum(digits: &Vec<u8>) -> u64 {
    let mut result: u64 = 0;
    for num in digits {
        let mut tmp_num: u64 = *num as u64;
        tmp_num *= 2;
        let multiplication_digits = digitize(tmp_num);
        for multiplicated_digit in multiplication_digits {
            result += multiplicated_digit as u64;
        }
    }
    return result;
}

fn sum_odd_digits(card_number_digits: &Vec<u8>) -> u64 {
    let mut result: u64 = 0;
    let num = card_number_digits.iter().enumerate().filter(|&x| x.0.is_odd());
    for x in num {
        let x = *x.1 as u64;
        result += x;
    }
    return result;
}

pub fn validate_card_number(card_number: u64) -> bool {
    let vec = digitize(card_number);

    let odd_sum = sum_odd_digits(&vec);
    let even_sum = two_times_each_and_sum(&even_digits(&vec));

    let luhn = odd_sum + even_sum;
    return luhn % 10 == 0;
}

#[test]
fn test_even_digits() {
    let result: Vec<u8> = vec![2, 4, 6, 8, 0];
    let card_number_digits: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    assert!(even_digits(&card_number_digits) == result);
}

#[test]
fn test_two_times_each_and_sum() {
    let test_data: Vec<u8> = vec![2, 4, 6, 8, 0];
    assert!(two_times_each_and_sum(&test_data) == 22);
}

#[test]
fn test_validation() {
    assert!(validate_card_number(49927398716));
    assert!(validate_card_number(49927398717) == false);
    assert!(validate_card_number(1234567812345678) == false);
    assert!(validate_card_number(1234567812345670) == false);
}
