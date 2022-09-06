#[cfg(test)]
mod tests {
    use rstest::rstest;
    use rstest::fixture;
    use algorithms::fib;
    use algorithms::fib_simple;

    type Functions = [fn(i32) -> i64; 2];

    #[fixture]
    // TODO: rewrite with parametrized fixture or dynamically parametrized tests
    fn test_functions() -> Functions {
        [fib, fib_simple]
    }

    #[rstest]
    #[case(1, 1)]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(5, 5)]
    #[case(10, 55)]
    #[case(20, 6765)]
    fn test_fibs(
        test_functions: Functions,
        #[case] arg: i32,
        #[case] expected: i64,
    ) {
        for f in test_functions {
            assert_eq!(f(arg), expected);
        }
    }
}
