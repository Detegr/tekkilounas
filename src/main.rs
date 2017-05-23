//! FizzBuzz program for fizzing and buzzing

#![feature(test)]
extern crate test;

use std::borrow::Cow;

/// Check whether a number is fizz
pub fn is_fizz(n: usize) -> bool {
    (n % 3) == 0
}

/// Check whether a number is buzz
pub fn is_buzz(n: usize) -> bool {
    (n % 5) == 0
}

/// Check whether a number is fizzbuzz
pub fn is_fizzbuzz(n: usize) -> bool {
    (n % 15) == 0
}

pub fn slow_fizzbuzz(n: usize) -> String {
    if is_fizzbuzz(n) {
        "fizzbuzz".into()
    } else if is_fizz(n) {
        "fizz".into()
    } else if is_buzz(n) {
        "buzz".into()
    } else {
        n.to_string()
    }
}

pub fn fizzbuzz<'a>(n: usize) -> Cow<'a, str> {
    if is_fizzbuzz(n) {
        "fizzbuzz".into()
    } else if is_fizz(n) {
        "fizz".into()
    } else if is_buzz(n) {
        "buzz".into()
    } else {
        Cow::Owned(n.to_string())
    }
}

#[bench]
fn bench_fizzbuzz(b: &mut test::Bencher) {
    b.iter(|| {
        for n in 0..1000 {
            test::black_box(fizzbuzz(n));
        }
    })
}

#[bench]
fn bench_fizzbuzz_slow(b: &mut test::Bencher) {
    b.iter(|| {
        for n in 0..1000 {
            test::black_box(slow_fizzbuzz(n));
        }
    })
}

mod toinen;

static VALUES: &'static [usize] = &[1, 3, 4, 6, 1, 3, 51, 42, 14, 123];

fn main() {
    let sum = toinen::sum_elements(VALUES, 4);
    println!("{:?}", sum);
}

#[test]
fn test_fizzbuzz() {
    assert!(is_fizzbuzz(15));
}
