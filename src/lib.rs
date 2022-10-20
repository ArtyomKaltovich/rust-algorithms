use std::cmp::Ordering;
use num::Integer;

pub fn bin_search<T: Ord>(array: &[T], elem: &T) -> Result<usize, usize>
{
    let mut l = 0;
    let mut r = array.len();
    while l < r {
        let i = (l + r) / 2;
        match &array[i].cmp(elem) {
            Ordering::Less => l = i + 1,
            Ordering::Equal => return Ok(i),
            Ordering::Greater => r = i,
        }
    }
    Err(r)
}

pub fn fib<T: Integer + Copy>(n: i32) -> T
{
    // uses recurrent equations derived from matrix form
    // https://en.wikipedia.org/wiki/Fibonacci_number#Matrix_form
    // https://www.nayuki.io/page/fast-fibonacci-algorithms
    if n <= 0 {panic!()};
    let mut a = T::zero();
    let mut b = T::one();
    let mut highest_power = 2_i32.pow(31 - n.leading_zeros());
    while highest_power > 0 {
        (a, b) = (a * (b + b - a), a * a + b * b);

        if n & highest_power != 0 {
            (a, b) = (b, a + b)
        }
        highest_power >>= 1;
    }
    return a;
}

pub fn fib_simple<T: Integer + Copy>(n: i32) -> T
{
    if n <= 0 {panic!()};
    let mut a = T::zero();
    let mut b = T::one();
    for _ in 1..n {
        (a, b) = (b, a + b)
    }
    return b;
}

pub fn insert_sort<T: Ord + Clone>(array: &[T]) -> Vec<T> {
    let mut result = array.to_vec();
    for i in 0..result.len() {
        let mut current = i;
        for j in (i + 1)..result.len() {
            if result[j] < result[current] {
                current = j;
            }
        }
        let temp = result[current].clone();
        result[current] = result[i].clone();
        result[i] = temp;
    }
    result
}

pub fn bubble_sort<T: Ord + Clone>(array: &[T]) -> Vec<T> {
    let mut result = array.to_vec();
    for i in 0..result.len() {
        for j in (i + 1)..result.len() {
            if result[i] > result[j] {
                let temp = result[j].clone();
                result[j] = result[i].clone();
                result[i] = temp;
            }
        }
    }
    result
}
