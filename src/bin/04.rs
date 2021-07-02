use std::convert::TryInto;

use itertools::any;

use aoc2019::*;

fn to_digits(x: &u32) -> Vec<u8> {
    fn inner(n: u32, ds: &mut Vec<u8>) {
        if n >= 10 {
            inner(n / 10, ds);
        }
        ds.push((n % 10).try_into().unwrap());
    }
    let mut digits = Vec::new();
    inner(*x, &mut digits);
    digits
}

fn digits_never_decrease(digits: &[u8]) -> bool {
    digits
        .iter()
        .fold((0, true), |(prev, never_decrease), curr| {
            (*curr, never_decrease && *curr >= prev)
        })
        .1
}

fn consecutive_digits(digits: &[u8]) -> Vec<u8> {
    digits
        .iter()
        .chain(std::iter::once(&10u8))
        .fold(
            (0, 1, Vec::new()),
            |(prev, consecutive, mut consecutives), curr| {
                if prev == *curr {
                    (*curr, consecutive + 1, consecutives)
                } else {
                    if consecutive > 1 {
                        consecutives.push(consecutive);
                    }
                    (*curr, 1, consecutives)
                }
            },
        )
        .2
}

fn part01(digits: &[Vec<u8>]) -> u32 {
    digits
        .iter()
        .filter(|ds| digits_never_decrease(ds) && any(consecutive_digits(ds), |d| d >= 2))
        .count()
        .try_into()
        .unwrap()
}

fn part02(digits: &[Vec<u8>]) -> u32 {
    digits
        .iter()
        .filter(|ds| digits_never_decrease(ds) && any(consecutive_digits(ds), |d| d == 2))
        .count()
        .try_into()
        .unwrap()
}

fn day_04() -> (u32, u32) {
    let nums: Vec<_> = get_input(4)
        .next()
        .unwrap()
        .split('-')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let digits = (nums[0]..=nums[1])
        .map(|x| to_digits(&x))
        .collect::<Vec<_>>();
    (part01(&digits), part02(&digits))
}

timed_main!(100, day_04());
