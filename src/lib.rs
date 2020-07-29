pub mod basic;
pub mod decimal;
pub mod hex;
pub mod stat;
pub mod time_series;
pub mod matrix;
pub mod ornstein_uhlenbeck_process;
pub mod probability_distribution;
pub mod vandermonde_matrix;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
