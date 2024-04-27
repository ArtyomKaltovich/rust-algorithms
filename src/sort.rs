use std::collections::HashMap;
use std::hash::Hash;

pub fn insert_sort<T: Ord + Clone>(array: &[T]) -> Vec<T> {
    let mut result = array.to_vec();
    for i in 0..result.len() {
        let mut current = i;
        for j in (i + 1)..result.len() {
            if result[j] < result[current] {
                current = j;
            }
        }
        result.swap(current, i);
    }
    result
}

pub fn bubble_sort<T: Ord + Clone>(array: &[T]) -> Vec<T> {
    let mut result = array.to_vec();
    for i in 0..result.len() {
        for j in (i + 1)..result.len() {
            if result[i] > result[j] {
                result.swap(i, j);
            }
        }
    }
    result
}

pub fn count_sort<T: Ord + Copy + Hash>(array: &[T]) -> Vec<T> {
    let mut result: Vec<T> = Vec::with_capacity(array.len());
    let hash_map = get_count_hash(array);
    let mut keys = hash_map.keys().collect::<Vec<&T>>();
    keys.sort();
    for k in keys {
        for _ in 0..*hash_map.get(k).unwrap() {
            result.push(*k);
        }
    }
    result
}

fn get_count_hash<T: Ord + Copy + Hash>(array: &[T]) -> HashMap<T, i32> {
    let mut hash_map: HashMap<T, i32> = HashMap::new();
    for a in array {
        if let Some(count) = hash_map.get_mut(a) {
            *count += 1;
        } else {
            hash_map.insert(*a, 1);
        }
    }
    hash_map
}
