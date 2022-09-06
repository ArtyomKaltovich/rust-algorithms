use std::time::Instant;
use algorithms::{fib, fib_simple};

fn main() {
    let t1 = Instant::now();
    for _ in 0..100 { // use cycle, because on 100 elements its overflows
        fib_simple(90);
    }
    let t2 = Instant::now();
    for _ in 0..100 {
        fib(90);
    }
    let t3 = t2.elapsed();
    println!("Elapsed on fib simple: {:.2?}", t2 - t1);
    println!("Elapsed on fib: {:.2?}", t3);
    // in debug fib is > 10x faster in release - slower :)
}
