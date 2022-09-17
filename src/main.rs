use std::time::Instant;
use algorithms::{fib, fib_simple};

fn main() {
    let t1 = Instant::now();
    let r = fib_simple::<i128>(180);
    let t2 = Instant::now();
    let r2 = fib::<i128>(180);
    let t3 = t2.elapsed();
    println!("fib simple = {}, time elapsed: {:.2?}", r, t2 - t1);
    println!("fib = {}, time elapsed: {:.2?}", r2, t3);
}
