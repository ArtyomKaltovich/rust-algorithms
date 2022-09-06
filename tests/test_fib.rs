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
    fn for_1(test_functions: Functions) {
        for f in test_functions {
            let result = f(1);
            assert_eq!(result, 1);
        }
    }

    #[rstest]
    fn for_2(test_functions: Functions) {
        for f in test_functions {
            let result = f(2);
            assert_eq!(result, 1);
        }
    }

    #[rstest]
    fn for_3(test_functions: Functions) {
        for f in test_functions {
            let result = f(3);
            assert_eq!(result, 2);
        }
    }

    #[rstest]
    fn for_5(test_functions: Functions) {
        for f in test_functions {
            let result = f(5);
            assert_eq!(result, 5);
        }
    }

    #[rstest]
    fn for_10(test_functions: Functions) {
        for f in test_functions {
            let result = f(10);
            assert_eq!(result, 55);
        }
    }

    #[rstest]
    fn for_20(test_functions: Functions) {
        for f in test_functions {
            let result = f(20);
            assert_eq!(result, 6765);
        }
    }
}
