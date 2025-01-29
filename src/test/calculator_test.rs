#[cfg(test)]
mod tests {
    use crate::calculator::Calculator;

    const DATA: [(f32, f32); 3] = [(1.0, 2.0), (-1.0, 2.0), (-1.0, -2.0)];

    fn setup() -> Calculator<f32> {
        Calculator { a: 0.0, b: 0.0 }
    }

    #[test]
    fn test_addition() {
        let mut calc = setup();
        for i in DATA {
            (calc.a, calc.b) = (i.0, i.1);
            let exp = i.0 + i.1;
            assert_eq!(exp, calc.add());
        }
    }

    #[test]
    fn test_subtraction() {
        let mut calc = setup();
        for i in DATA {
            (calc.a, calc.b) = (i.0, i.1);
            let exp = i.0 - i.1;
            assert_eq!(exp, calc.sub());
        }
    }

    #[test]
    fn test_multiplication() {
        let mut calc = setup();
        for i in DATA {
            (calc.a, calc.b) = (i.0, i.1);
            let exp = i.0 * i.1;
            assert_eq!(exp, calc.mul());
        }
    }

    #[test]
    fn test_division() {
        let mut calc = setup();
        for i in DATA {
            (calc.a, calc.b) = (i.0, i.1);
            let exp = i.0 / i.1;
            assert_eq!(exp, calc.div());
        }
    }
}
