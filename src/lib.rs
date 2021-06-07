mod q8;
pub use q8::I32Q8;

pub trait FixedPoint {
    const SHIFT: usize;

    fn as_f64(&self) -> f64;

    fn delta() -> f64 {
        1. / (1 << Self::SHIFT) as f64
    }
}

#[cfg(test)]
mod tests {
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }
}
