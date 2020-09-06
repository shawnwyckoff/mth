use std::prelude::v1::Iterator;

/// Basic statistics functions
pub trait StatBasic {
    /// Result type
    type Result;

    /// Compute arithmetic mean
    /// (reference)[http://en.wikipedia.org/wiki/Arithmetic_mean]
    fn u_mean(self) -> Self::Result;

    /// Compute standard deviation
    fn u_stddev(self) -> Self::Result;

    /// Compute statistical variance
    fn u_variance(self) -> Self::Result;
}

impl StatBasic for &[f64] {
    type Result = f64;

    fn u_mean(self) -> f64 {
        let mut len: f64 = 0.0;
        return self.iter().fold(0.0, |acc: f64, ele| {
            len += 1.0;
            return acc + *ele;
        }) / len;
    }

    fn u_stddev(self) -> f64 {
        return self.u_variance().sqrt();
    }

    fn u_variance(self) -> f64 {
        let mean = self.clone().u_mean();
        let mut len: f64 = 0.0;
        let mut n: f64 = 0.0;
        return self.iter().fold(0.0, |sum, ele| {
            len += 1.0;
            n = *ele - mean;
            return sum + (n * n);
        }) / (len - 1.0);
    }
}

/*
impl<I> StatBasic for I
    where
        I: Iterator<Item = f64> + Iterator<Item = Decimal> + Clone,
{
    type Result = f64;

    fn u_mean(self) -> f64 {
        let mut len: f64 = 0.0;
        return self.fold(0.0, |acc: f64, x| {len += 1.0; return acc + x; }) / len;
    }

    fn u_stddev(self) -> f64 {
        return self.u_variance().sqrt();
    }

    fn u_variance(self) -> f64 {
        let mean = self.clone().u_mean();
        let mut len: f64 = 0.0;
        let mut n: f64 = 0.0;
        return self.fold(0.0, |sum, ele| {len += 1.0; n = ele - mean; return sum + (n * n);}) / (len - 1.0);
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;
    use num::abs;

    const EPSILON: f64 = 1e-6;

    #[test]
    fn u_mean_test() {
        assert_eq!([1.0, 3.0, 5.0].as_ref().u_mean(), 3.0);

        let diff = abs([0.0, 0.25, 0.25, 1.25, 1.5, 1.75, 2.75, 3.25]
            .as_ref()
            .u_mean()
            - 1.375);
        assert!(diff <= EPSILON);
    }

    #[test]
    fn u_variance_test() {
        assert_eq!([1.0, 3.0, 5.0].as_ref().u_variance(), 4.0);
    }

    #[test]
    fn u_stddev_test() {
        assert_eq!([1.0, 3.0, 5.0].as_ref().u_stddev(), 2.0);
    }
}
