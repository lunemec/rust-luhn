// https://play.rust-lang.org/?code=%23!%5Bfeature(zero_one)%5D%0A%0Ause%20std%3A%3Anum%3A%3A%7BZero%2C%20One%7D%3B%0Ause%20std%3A%3Aops%3A%3A%7BAdd%2C%20Sub%2C%20Mul%2C%20Rem%7D%3B%0A%0Afn%20is_prime%3CN%3A%20Copy%20%2B%20Zero%20%2B%20One%20%2B%20PartialEq%20%2B%20PartialOrd%20%2B%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20Add%3CN%2C%20Output%20%3D%20N%3E%20%2B%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20Rem%3CN%2C%20Output%20%3D%20N%3E%20%2B%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20Mul%3CN%2C%20Output%20%3D%20N%3E%20%2B%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20Sub%3CN%2C%20Output%20%3D%20N%3E%3E(n%3A%20N)%20-%3E%20bool%20%7B%0A%20%20%20%20let%20_0%20%3D%20N%3A%3Azero()%3B%0A%20%20%20%20let%20_1%20%3D%20N%3A%3Aone()%3B%0A%20%20%20%20let%20_2%20%3D%20_1%20%2B%20_1%3B%0A%20%20%20%20let%20_3%20%3D%20_2%20%2B%20_1%3B%0A%20%20%20%20let%20_5%20%3D%20_2%20%2B%20_3%3B%0A%20%20%20%20let%20_6%20%3D%20_3%20%2B%20_3%3B%0A%20%20%20%20if%20n%20%3D%3D%20_2%20%7C%7C%20n%20%3D%3D%20_3%20%7B%0A%20%20%20%20%20%20%20%20return%20true%3B%0A%20%20%20%20%7D%20else%20if%20n%20%25%20_2%20%3D%3D%20_0%20%7C%7C%20n%20%25%20_3%20%3D%3D%20_0%20%7B%0A%20%20%20%20%20%20%20%20return%20false%3B%0A%20%20%20%20%7D%0A%0A%20%20%20%20let%20mut%20i%20%3D%20_5%3B%0A%20%20%20%20let%20mut%20w%20%3D%20_2%3B%0A%20%20%20%20while%20i%20*%20i%20%3C%3D%20n%20%7B%0A%20%20%20%20%20%20%20%20if%20n%20%25%20i%20%3D%3D%20_0%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20return%20false%3B%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%20%20%20%20i%20%3D%20i%20%2B%20w%3B%0A%20%20%20%20%20%20%20%20w%20%3D%20_6%20-%20w%3B%0A%20%20%20%20%7D%0A%20%20%20%20true%0A%7D%0A%0A%23%5Btest%5D%0Afn%20test_primes()%20%7B%0A%20%20%20%20assert!(is_prime(17usize))%3B%0A%20%20%20%20assert!(is_prime(13u32))%3B%0A%20%20%20%20assert!(!is_prime(9i16))%3B%0A%20%20%20%20assert!(!is_prime(4i8))%3B%0A%7D&version=nightly
// https://stackoverflow.com/questions/26810793/how-can-i-create-an-is-prime-function-that-is-generic-over-various-integer-types
// https://github.com/rust-num/num

use std::clone::Clone;
use std::num::{Zero, One};
use std::ops::{Add, Div, DivAssign, Mul, Rem};
use std::cmp::Ord;

fn digitize<N: Copy + Clone + PartialOrd + One + Zero + DivAssign +
            Add<N, Output = N> +
            Mul<N, Output = N> +
            Div<N, Output = N> +
            Rem<N, Output = N>>(number: N) -> Vec<N> {
    let mut number = number.clone();
    let mut digits: Vec<N> = Vec::new();

    let _0 = N::zero();
    let _1 = N::one();
    let _2 = _1 + _1;
    let _3 = _1 + _2;
    let _5 = _2 + _3;
    let _10 =  _5 * _2;
    while number > _0 {
        let remainder = number % _10;
        digits.push(remainder);
        number /= _10;
    }
    return digits;
}

fn main() {
    let test = vec![1, 2, 3, 4];
    println!("{:?}", test[0]);
}
