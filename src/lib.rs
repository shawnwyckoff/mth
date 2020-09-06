pub mod erasure_code;
pub mod basic;
pub mod galois_field;
pub mod matrix;
pub mod number;
pub mod ornstein_uhlenbeck_process;
pub mod polynomial;
pub mod price_distance;
pub mod probability_distribution;
pub mod single_linked_list;
pub mod stat;
pub mod vandermonde_matrix;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
