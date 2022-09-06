use std::cmp::Ordering;

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

pub fn fib(n: i32) -> i64
{
    // uses recurrent equations derived from matrix form
    // https://en.wikipedia.org/wiki/Fibonacci_number#Matrix_form
    // https://www.nayuki.io/page/fast-fibonacci-algorithms
    if n <= 0 {panic!()};
    let mut a = 0;
    let mut b = 1;
    let mut highest_power = 2_i32.pow(31 - n.leading_zeros());
    while highest_power > 0 {
        (a, b) = (a * (2 * b - a), a * a + b * b);

        if n & highest_power != 0 {
            (a, b) = (b, a + b)
        }
        highest_power >>= 1;
    }
    return a;
}

pub fn fib_simple(n: i32) -> i64
{
    if n <= 0 {panic!()};
    let mut a = 0;
    let mut b = 1;
    for _ in 1..n {
        (a, b) = (b, a + b)
    }
    return b;
}
