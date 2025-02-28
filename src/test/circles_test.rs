#[cfg(test)]
mod tests {
    use crate::circles::{area, perimeter};
    use std::f32::consts::PI;

    fn get_data() -> [f32; 4] {
        [0.0, 5.0, -5.0, -0.0]
    }

    #[test]
    fn test_circle_area() {
        let input = get_data();
        for r in input {
            let exp = {
                if r > 0.0 {
                    PI * r.powi(2)
                } else {
                    0.0
                }
            };
            let res = area(r).unwrap_or_default();
            assert_eq!(exp, res);
        }
    }

    #[test]
    fn test_circle_perim() {
        let input = get_data();
        for r in input {
            let exp = {
                if r > 0.0 {
                    2.0 * PI * r
                } else {
                    0.0
                }
            };
            let res = perimeter(r).unwrap_or_default();
            assert_eq!(exp, res);
        }
    }
}
