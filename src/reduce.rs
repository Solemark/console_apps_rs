pub struct Reducer<N> {
    data: Vec<N>,
}

#[allow(dead_code)]
impl<
        N: std::ops::Add<Output = N>
            + std::ops::Sub<Output = N>
            + std::ops::Mul<Output = N>
            + std::ops::Div<Output = N>
            + std::clone::Clone,
    > Reducer<N>
{
    pub fn new(data: Vec<N>) -> Self {
        Reducer { data }
    }

    pub fn add(&self) -> N {
        self.data.clone().into_iter().reduce(|a, b| a + b).unwrap()
    }

    pub fn sub(&self) -> N {
        self.data.clone().into_iter().reduce(|a, b| a - b).unwrap()
    }

    pub fn mul(&self) -> N {
        self.data.clone().into_iter().reduce(|a, b| a * b).unwrap()
    }

    pub fn div(&self) -> N {
        self.data.clone().into_iter().reduce(|a, b| a / b).unwrap()
    }

    pub fn other(&self, func: fn(N, N) -> N) -> N {
        self.data.clone().into_iter().reduce(func).unwrap()
    }
}
