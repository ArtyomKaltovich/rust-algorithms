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
