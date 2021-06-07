use crate::FixedPoint;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct I32Q8(pub i32);

impl I32Q8 {
    pub fn from_i32(x: i32) -> Self {
        Self(x << 8)
    }

    pub fn mul_with_round_left(&self, right: Self) -> Self {
        let Self(left) = *self;
        let Self(right) = right;

        Self(Self::round(left) * right)
    }

    pub fn mul_with_round_right(&self, right: Self) -> Self {
        let Self(left) = self;
        let Self(right) = right;
        Self(left * Self::round(right))
    }

    fn round(x: i32) -> i32 {
        let half = 1 << (Self::SHIFT - 1);
        if x >= 0 {
            (x + half) >> Self::SHIFT
        } else {
            (x - half) >> Self::SHIFT
        }
    }
}

impl FixedPoint for I32Q8 {
    const SHIFT: usize = 8;

    fn as_f64(&self) -> f64 {
        let Self(x) = self;
        return *x as f64 / (1 << Self::SHIFT) as f64;
    }
}

impl From<I32Q8> for f64 {
    fn from(x: I32Q8) -> f64 {
        x.as_f64()
    }
}

impl From<i32> for I32Q8 {
    fn from(x: i32) -> Self {
        Self::from_i32(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expect = 0.8984375;
        let x = I32Q8(0b0_0000000_11100110);
        let y: f64 = x.into();
        assert_eq!(y, expect);

        let expect = 1.8984375;
        let x = I32Q8(0b0_0000001_11100110);
        let y: f64 = x.into();
        assert_eq!(y, expect);

        assert_eq!(I32Q8::delta(), 0.00390625);
    }

    #[test]
    fn from_i32_test() {
        let expect = I32Q8(1 << 8);
        let x: I32Q8 = 1i32.into();
        assert_eq!(x, expect);
    }
}
