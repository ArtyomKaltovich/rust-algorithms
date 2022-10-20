#[cfg(test)]
mod tests {
    use rstest::rstest;
    use algorithms::{insert_sort, bubble_sort};

    #[rstest]
    #[case(&[1], &[1])]
    #[case(&[2, 1], &[1, 2])]
    #[case(&[1, 2], &[1, 2])]
    #[case(&[5, 5], &[5, 5])]
    #[case(&[1, 3, 2], &[1, 2, 3])]
    #[case(&[1, 0, -1], &[-1, 0, 1])]
    #[case(&[5, 1, 1, -1], &[-1, 1, 1, 5])]
    fn test_sort(
        #[values(insert_sort, bubble_sort)] sort_function: fn(&[i32]) -> Vec<i32>,
        #[case] array: &[i32],
        #[case] expected: &[i32],
    ) {
        let actual = insert_sort(array);
        assert_eq!(expected, actual);
    }
}
