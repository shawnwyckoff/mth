/// x: x axis
///
/**
$$
f(x)=\frac{1}{\sqrt{2 \pi} \sigma} e^{-\frac{(x-\mu)^{2}}{2 \sigma^{2}}}
$$
*/
pub fn normal_distribution(x: f64, mu: f64, sigma: f64) -> f64 {
    (std::f64::consts::E.powf(-((x - mu).powi(2) / (2.0 * sigma * sigma))))
        / ((std::f64::consts::PI * 2.0).sqrt() * sigma)
}

pub fn standard_normal_distribution(x: f64) -> f64 {
    normal_distribution(x, 0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use crate::probability_distribution::*;

    #[test]
    fn normal_distribution_test() {
        let y = normal_distribution(1.0, 2.0, 3.0);
        println!("{}", y)
    }

    #[test]
    fn standard_normal_distribution_test() {
        let y = standard_normal_distribution(1.0);
        println!("{}", y)
    }
}
