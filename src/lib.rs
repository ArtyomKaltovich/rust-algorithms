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
        let d = a * (2 * b - a);
        let e = a * a + b * b;
        a = d;
        b = e;

        if n & highest_power != 0 {
            let c = a + b;
            a = b;
            b = c;
        }
        highest_power >>= 1;
    }
    return a;
}
