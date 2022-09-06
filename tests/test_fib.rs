#[cfg(test)]
mod tests {
    use rstest::rstest;
    use algorithms::{fib, fib_simple};

    #[rstest]
    #[case(1, 1)]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(5, 5)]
    #[case(10, 55)]
    #[case(20, 6765)]
    fn test_fibs(
        #[values(fib, fib_simple)] f: fn(i32) -> i64,
        #[case] arg: i32,
        #[case] expected: i64,
    ) {
        assert_eq!(f(arg), expected);
    }
}
