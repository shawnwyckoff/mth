use crate::matrix::matrix::Matrix;
use crate::galois_field::gf_u8::Gf2p;
use std::borrow::Borrow;

pub struct ReedSolomon {
    gm: Matrix<u8>,
    gf: Gf2p,
}

impl ReedSolomon {
    pub fn new(dataNum: u8, parityNum: u8) -> anyhow::Result<ReedSolomon> {
        let gf = Gf2p::new(8, 0x1D)?;
        let im = Matrix::new_identity_matrix(dataNum as usize)?;
        let cm = Matrix::new_cauchy_matrix(&gf, parityNum as usize, dataNum as usize)?;
        let gm = im.append_bottom(cm)?;
        Ok(ReedSolomon {
            gm: gm,
            gf: gf,
        })
    }

    pub fn encode(self, data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let mut gm = self.gm.clone();

        if data.len() != gm.get_col_size() {
            return Err(anyhow::anyhow!("data size must equals to generate column size"));
    }
        let dm = Matrix::new_column_vector(data);

        let mut gm = self.gm.clone();

        let out = gm.mul_gf(dm, self.gf);
        if out.is_err() {
            return Err(anyhow::anyhow!("gf mul error"));
        }
        Ok(out.unwrap().to_vector_u8())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::galois_field::gf_u8::print_matrix_u8;

    #[test]
    fn encode() {
        let data: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x10];

        let mut rs_res = ReedSolomon::new(data.len() as u8, 2);
        if rs_res.is_err() {
        }
        let mut rs = rs_res.unwrap();

        let enc_res = rs.encode(data);
        let is_err = enc_res.is_err();
        if is_err {
            println!("{}", enc_res.err().unwrap());
            assert_ne!(is_err, true);
        }
        //let enc_data = enc_res.unwrap();
        //print_matrix_u8(enc_data);
    }
}