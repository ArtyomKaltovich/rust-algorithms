use std::time::Instant;
use algorithms::sort::{bubble_sort, insert_sort};
use rand::prelude::*;

fn main() {
    run_sort();
}

fn run_sort() {
    let mut nums: Vec<i32> = (1..10_000).collect();
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);
    let mut array = nums.clone();
    let mut array2 = nums.clone();

    let t = Instant::now();
    insert_sort(&mut array.as_mut_slice());
    let e_insert = t.elapsed();

    let t = Instant::now();
    bubble_sort(&mut array2.as_mut_slice());
    let e_bubble = t.elapsed();
    println!("insert sort simple time elapsed: {:.2?}", e_insert);
    println!("bubble sort simple time elapsed: {:.2?}", e_bubble);
}
