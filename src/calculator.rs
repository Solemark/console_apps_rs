pub struct Calculator<N> {
    pub a: N,
    pub b: N,
}

#[allow(dead_code)]
impl<
        N: std::ops::Add<Output = N>
            + std::ops::Sub<Output = N>
            + std::ops::Mul<Output = N>
            + std::ops::Div<Output = N>
            + std::clone::Clone,
    > Calculator<N>
{
    pub fn new(a: N, b: N) -> Self {
        Calculator { a, b }
    }

    pub fn add(&self) -> N {
        self.a.clone() + self.b.clone()
    }

    pub fn sub(&self) -> N {
        self.a.clone() - self.b.clone()
    }

    pub fn mul(&self) -> N {
        self.a.clone() * self.b.clone()
    }

    pub fn div(&self) -> N {
        self.a.clone() / self.b.clone()
    }
}
