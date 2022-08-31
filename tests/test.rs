#[cfg(test)]
mod tests {
    use algorithms::bin_search;

    #[test]
    fn it_works() {
        let result = bin_search(&[1, 2, 3, 4, 5], &3);
        assert_eq!(result, Result::Ok(2));
    }

    #[test]
    fn it_works_first() {
        let result = bin_search(&[1, 2, 3, 4, 5], &1);
        assert_eq!(result, Result::Ok(0));
    }

    #[test]
    fn it_works_last() {
        let result = bin_search(&[1, 2, 3, 4, 5], &5);
        assert_eq!(result, Result::Ok(4));
    }

    #[test]
    fn it_works_series() {
        let result = bin_search(&[1, 2, 3, 3, 4, 5], &3);
        assert! (result == Result::Ok(2) || result == Result::Ok(3));
    }

    #[test]
    fn empty_array() {
        let result = bin_search(&[], &"a");
        assert_eq!(result, Result::Err(0));
    }

    #[test]
    fn not_found_last() {
        let result = bin_search(&["a", "b", "b"], &"c");
        assert_eq!(result, Result::Err(3));
    }

    #[test]
    fn not_found_first() {
        let result = bin_search(&["a", "b", "b"], &"A");
        assert_eq!(result, Result::Err(0));
    }

    #[test]
    fn not_found_middle() {
        let result = bin_search(&["a", "c", "d"], &"b");
        assert_eq!(result, Result::Err(1));
    }
}
