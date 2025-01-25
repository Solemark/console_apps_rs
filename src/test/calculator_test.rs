#[cfg(test)]
mod tests {
    use crate::calculator::{add, div, mul, sub};

    const DATA: [(f32, f32); 3] = [(1.0, 2.0), (-1.0, 2.0), (-1.0, -2.0)];

    #[test]
    fn test_addition() {
        for i in DATA {
            let exp = i.0 + i.1;
            let res = add(i.0, i.1);
            assert_eq!(exp, res);
        }
    }

    #[test]
    fn test_subtraction() {
        for i in DATA {
            let exp = i.0 - i.1;
            let res = sub(i.0, i.1);
            assert_eq!(exp, res);
        }
    }

    #[test]
    fn test_multiplication() {
        for i in DATA {
            let exp = i.0 * i.1;
            let res = mul(i.0, i.1);
            assert_eq!(exp, res);
        }
    }

    #[test]
    fn test_division() {
        for i in DATA {
            let exp = i.0 / i.1;
            let res = div(i.0, i.1);
            assert_eq!(exp, res);
        }
    }
}
