#[cfg(test)]
mod tests {
    use crate::reduce::Reducer;

    #[test]
    fn test_add() {
        let red = Reducer::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(55, red.add());
    }

    #[test]
    fn test_sub() {
        let red = Reducer::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(-53, red.sub());
    }

    #[test]
    fn test_mul() {
        let red = Reducer::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(3628800, red.mul());
    }

    #[test]
    fn test_div() {
        let red = Reducer::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(0, red.div());
    }

    #[test]
    fn test_other() {
        let red = Reducer::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(100, red.other(other_reduce));
    }

    fn other_reduce(a: i32, b: i32) -> i32 {
        (a - 1) + (b * 2)
    }
}
