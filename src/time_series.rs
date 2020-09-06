use std::collections::{BTreeSet, HashMap};
use time::Time;

pub struct TimeSeries {
    t: BTreeSet<time::Time>, // times
    v: Vec<Vec<f64>>,        // values matrix
    n: [String],             // name index
}

impl TimeSeries {
    fn upsert_val(&self, t: time::Time, v: f64, n: &str) {}

    fn upsert_vals(&self, t: time::Time, vn_map: HashMap<&str, f64>) {}

    fn get_by_name(&self, name: &str) -> Result<Vec<f64>, anyhow::Error> {
        Err(anyhow::anyhow!("none"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_series_get_by_name_test() {}
}
