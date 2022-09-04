#[cfg(test)]
mod tests {
    use algorithms::fib;

    #[test]
    fn for_1() {
        let result = fib(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn for_2() {
        let result = fib(2);
        assert_eq!(result, 1);
    }

    #[test]
    fn for_3() {
        let result = fib(3);
        assert_eq!(result, 2);
    }

    #[test]
    fn for_5() {
        let result = fib(5);
        assert_eq!(result, 5);
    }

    #[test]
    fn for_10() {
        let result = fib(10);
        assert_eq!(result, 55);
    }

    #[test]
    fn for_20() {
        let result = fib(20);
        assert_eq!(result, 6765);
    }
}
