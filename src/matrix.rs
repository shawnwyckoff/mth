pub struct matrix {
    v: Vec<Vec<f64>>, // values matrix
    n: [String], // name index
}

/**

$$
s_{i t}=\ln P_{i t}-\ln P_{i 0}
$$

$$
D_{i j}=\sum_{t=1}^{T}\left(s_{i t}-s_{j t}\right)^{2}
$$

*/
pub fn price_distance(val_from: &[f64], val_to: &[f64]) -> Result<f64, anyhow::Error> {
    if val_from.len() != val_to.len() {
        return Err(anyhow::anyhow!("from length {} != to length {}", val_from.len(), val_to.len()));
    }
    let zero_usize : usize = 0;
    if val_from.len().eq(&zero_usize) {
        return Err(anyhow::anyhow!("empty data length"));
    }
    let mut res: f64 = 0.0;
    let pi0_from = val_from[0].ln();
    let pi0_to = val_to[0].ln();
    for i in 0..val_from.len() {
        res += ((val_from[i].ln() - pi0_from) - (val_to[i].ln() - pi0_to)).powi(2)
    }
    return Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn price_distance_test() {
        let left = [1.0, 2.0, 3.0, 4.0, 5.0].as_ref();
        let right = [2.0, 4.0, 4.0, 5.0, 7.0].as_ref();
        let res = price_distance(left, right);
        assert!(res.is_ok());
        println!("{}", res.unwrap());
    }
}