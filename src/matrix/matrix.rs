/**
First version of this matrix implement is based on github.com/whostolemyhat/learning-projects .
*/
extern crate rand;

use rand::Rng;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Mul, Sub};
use self::rand::distributions::uniform::SampleUniform;
use crate::galois_field::gf_u8::Gf2p;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    row_size: usize,
    col_size: usize,
    data: Vec<Vec<T>>,
}


type MatrixU8 = Matrix<u8>;

impl<T> Matrix<T>
where
    T: Copy + Default + SampleUniform,
{
    pub fn new(row_size: usize, col_size: usize, val: T) -> Self {
        let mut data: Vec<Vec<T>> = Vec::new();

        for _ in 0..row_size {
            let row = vec![val; col_size];
            data.push(row);
        }

        Matrix {
            row_size: row_size,
            col_size: col_size,
            data: data,
        }
    }

    fn new_random(row_size: usize, col_size: usize, low: T, high: T) -> Matrix<T> {
        let mut data: Vec<Vec<T>> = Vec::new();

        for _ in 0..row_size {
            let mut row: Vec<T> = Vec::new();

            for _ in 0..col_size {
                row.push(rand::thread_rng().gen_range(low, high));
            }

            data.push(row);
        }

        Matrix {
            row_size: row_size,
            col_size: col_size,
            data: data,
        }
    }

    pub fn new_column_vector(input: Vec<T>) -> Self {
        let mut data: Vec<Vec<T>> = Vec::new();
        for i in 0..input.len() {
            data.push(vec![input[i]; 1]);
        }

        Matrix {
            row_size: input.len(),
            col_size: 1,
            data: data,
        }
    }

    pub fn new_from_vec(data: Vec<Vec<T>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();

        Matrix {
            row_size: rows,
            col_size: cols,
            data: data,
        }
    }

    pub fn to_vector(self) -> Vec<T> {
        let mut vec :Vec<T> = Vec::new();
        for row in 0..self.row_size {
            for col in 0..self.col_size {
                vec.push(self.data[row][col]);
            }
        }
        vec
    }

    pub fn append_left(mut self, to_append: Matrix<T>) -> anyhow::Result<()> {
        if to_append.row_size != self.row_size {
            return Err(anyhow::anyhow!(
                "self.row_size ".to_string() + self.row_size.to_string().as_str() + " != to_append.row_size " + to_append.row_size.to_string().as_str()
            ));
        }
        if to_append.row_size == 0 || to_append.col_size == 0 {
            return Ok(());
        }

        let mut new_matrix = Matrix::new(self.row_size, self.col_size + to_append.col_size, to_append.data[0][0]);
        self = new_matrix;
        return Ok(());
    }

    /*pub fn mul_by_gf(self, gf: Gf<T>, multiplier: Matrix<T>) -> anyhow::Result<Matrix<T>> {
        return Err(anyhow::anyhow!(""));
    }*/

    pub fn scalar_mul(self, multiplier: T) -> Self
    where
        T: Mul<Output = T> + Copy + Default,
    {
        let mut new_matrix = Matrix::new(self.row_size, self.col_size, Default::default());

        for row in 0..self.row_size {
            for col in 0..self.col_size {
                new_matrix.data[row][col] = self.data[row][col].clone() * multiplier.clone();
            }
        }

        new_matrix
    }

    pub fn transpose(self) -> Self {
        let mut new_matrix = Matrix::new(self.col_size, self.row_size, Default::default());
        for row in 0..self.row_size {
            for col in 0..self.col_size {
                new_matrix.data[col][row] = self.data[row][col];
            }
        }

        new_matrix
    }
}

// Make sure generic type T implements Add (so you can add them together)
// Copy so we can copy self.rows/self.cols to new matrix
// and Default, so we can use that to fill the matrix
// <Output=T> ensures the T implementation returns a T
impl<T> Add for Matrix<T>
where
    T: Add<Output = T> + Copy + Default + SampleUniform,
{
    type Output = Matrix<T>;

    fn add(self, other: Matrix<T>) -> Matrix<T> {
        let mut new_matrix = Matrix::new(self.row_size, self.col_size, Default::default());

        for row in 0..self.row_size {
            for col in 0..self.col_size {
                new_matrix.data[row][col] = self.data[row][col] + other.data[row][col];
            }
        }

        new_matrix
    }
}

impl<T> Sub for Matrix<T>
where
    T: Sub<Output = T> + Copy + Default + SampleUniform,
{
    type Output = Matrix<T>;

    fn sub(self, other: Matrix<T>) -> Matrix<T> {
        let mut new_matrix: Matrix<T> =
            Matrix::new(self.row_size, self.col_size, Default::default());

        for row in 0..self.row_size {
            for col in 0..self.col_size {
                new_matrix.data[row][col] = self.data[row][col] - other.data[row][col];
            }
        }

        new_matrix
    }
}

impl<T> Mul for Matrix<T>
where
    T: Mul<Output = T> + Copy + Default + Add<Output = T> + SampleUniform,
{
    type Output = Matrix<T>;

    // this is dot product
    fn mul(self, other: Matrix<T>) -> Matrix<T> {
        // TODO: add a check for other.rows and self.cols (currently panics)
        // other.rows must eq self.cols
        // size = self.rows x other.cols

        // http://www.freemathhelp.com/matrix-multiplication.html
        let mut new_matrix: Matrix<T> =
            Matrix::new(self.row_size, other.col_size, Default::default());

        for i in 0..self.row_size {
            for j in 0..other.col_size {
                let mut total: T = Default::default();
                for k in 0..other.row_size {
                    total = total + self.data[i][k] * other.data[k][j];
                }
                // each number in self.row * each number in other.col
                new_matrix.data[i][j] = total;
            }
        }

        new_matrix
    }
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        for row in 0..self.row_size {
            for col in 0..self.col_size {
                write!(f, "{} ", self.data[row][col])?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}



impl MatrixU8 {
    // Create cauchy matrix by galois field and matrix size.
    // field: galois field this cauchy matrix based on.
    // row_size: row size.
    // col_size: column size.
    // this function will not store gf
    pub fn new_cauchy_matrix(
        gf: &Gf2p,
        row_size: usize,
        col_size: usize,
    ) -> anyhow::Result<Self> {
        if row_size > 0xFF || col_size > 0xFF {
            // row_size > u8 or col_size > u8
            return Err(anyhow::anyhow!(
                "row_size/col_size in cauchy matrix must <= u8"
            ));
        }
        if row_size + col_size > gf.get_element_count() {
            return Err(anyhow::anyhow!(
                "row_size + col_size must <= filed element count in cauchy matrix"
            ));
        }
        let startEle = 1;
        let xSize = row_size; // count of element X_i is row size
        let ySize = col_size; // count of element Y_i is col size
        let mut xSet = vec![0u8; xSize];
        let mut ySet = vec![0u8; ySize];
        for i in 0..xSize {
            xSet[i] = startEle + i as u8;
        }
        for j in 0..ySize {
            ySet[j] = startEle + xSize as u8 + j as u8;
        }

        /*xSet[0] = 1;
        xSet[1] = 2;
        ySet[0] = 0;
        ySet[1] = 3;
        ySet[2] = 4;
        ySet[3] = 5;
        ySet[4] = 6;*/

        let mut new_matrix = Matrix::new(row_size, col_size, 0u8);

        for row in 0..row_size {
            for col in 0..col_size {
                new_matrix.data[row][col] =
                    gf.div_by_power_log_table(1, gf.add(xSet[row], ySet[col]));
            }
        }

        Ok(new_matrix)
    }

    pub fn new_identity_matrix(size: usize) ->anyhow::Result<MatrixU8> {
        if size == 0 {
            return Err(anyhow::anyhow!("matrix size must > 0"));
        }
        let mut new_matrix = Matrix::new(size, size, 0u8);
        for i in 0..size {
            new_matrix.data[i][i] = 1;
        }
        return Ok(new_matrix);
    }

    pub fn append_bottom(self, to_append: MatrixU8) ->anyhow::Result<MatrixU8, anyhow::Error> {
        if self.col_size != to_append.col_size {
            return Err(anyhow::anyhow!("col size must be equal"))
        }
        let mut new_matrix = Matrix::new(self.row_size + to_append.row_size, self.col_size, 0u8);
        for i in 0..self.row_size {
            new_matrix.data[i] = self.data[i].clone();
        }
        for j in 0..to_append.row_size {
            new_matrix.data[j + self.row_size] = to_append.data[j].clone();
        }

        return Ok(new_matrix);
    }

    pub fn to_vector_u8(self) -> Vec<u8> {
        let mut vec :Vec<u8> = Vec::new();
        for row in 0..self.row_size {
            for col in 0..self.col_size {
                vec.push(self.data[row][col]);
            }
        }
        vec
    }

    pub fn get_row_size(self) ->usize {
        self.row_size
    }

    pub fn get_col_size(self) ->usize {
        self.col_size
    }

    /**
    $$
    (A B)_{i j}=\sum_{k=1}^{p} a_{i k} b_{k j}=a_{i 1} b_{1 j}+a_{i 2} b_{2 j}+\cdots+a_{i p} b_{p j}
    $$
    */
    pub fn mul_gf(self, multiplier: MatrixU8, gf: Gf2p) -> anyhow::Result<MatrixU8> {
        if self.col_size != multiplier.row_size {
            return Err(anyhow::anyhow!("A col size must equal to B row size"));
        }

        let m = self.row_size;
        let p = self.col_size;
        let n = multiplier.col_size;
        let mut new_matrix = Matrix::new(m, n, 0u8);
        for mkey in 0..m {
            for nkey in 0..n {
                let mut sum = 0u8;
                for pkey in 0..p {
                    sum = gf.add(sum, gf.mul_by_power_log_table(self.data[mkey][pkey], multiplier.data[nkey][pkey]));
                }
                new_matrix.data[mkey][nkey] = sum;
            }
        }

        return Ok(new_matrix);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn Matrix_create_test() {
        let m1 = Matrix::new(2, 2, 2);
        let matrix = vec![vec![2, 2], vec![2, 2]];

        assert_eq!(m1.row_size, 2);
        assert_eq!(m1.col_size, 2);
        assert_eq!(m1.data, matrix);

        let mut m2 = Matrix::new(2, 4, 1);
        let second_matrix = vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1]];

        assert_eq!(m2.row_size, 2);
        assert_eq!(m2.col_size, 4);
        assert_eq!(m2.data, second_matrix);

        m2.data[1][1] = 5;
        let changed_matrix = vec![vec![1, 1, 1, 1], vec![1, 5, 1, 1]];
        assert_eq!(m2.data, changed_matrix);

        let ran = Matrix::new_random(2, 5, -20i32, 20i32);
        assert_eq!(ran.row_size, 2);
        assert_eq!(ran.col_size, 5);

        let third_matrix = vec![vec![13, 9, 7, 15], vec![8, 7, 4, 6], vec![6, 4, 0, 3]];
        let m3 = Matrix::new_from_vec(vec![vec![13, 9, 7, 15], vec![8, 7, 4, 6], vec![6, 4, 0, 3]]);
        assert_eq!(m3.row_size, 3);
        assert_eq!(m3.col_size, 4);
        assert_eq!(m3.data, third_matrix);
    }

    #[test]
    fn Matrix_add_test() {
        let m1 = Matrix::new(3, 3, 1);
        let m2 = Matrix::new(3, 3, 2);
        let m3 = m1 + m2;
        let matrix = vec![vec![3, 3, 3], vec![3, 3, 3], vec![3, 3, 3]];

        assert_eq!(m3.row_size, 3);
        assert_eq!(m3.col_size, 3);
        assert_eq!(m3.data, matrix);
    }

    #[test]
    fn Matrix_sub_test() {
        let m1 = Matrix::new(3, 3, 1);
        let m2 = Matrix::new(3, 3, 2);
        let matrix = vec![vec![-1, -1, -1], vec![-1, -1, -1], vec![-1, -1, -1]];
        let m3 = m1 - m2;

        assert_eq!(m3.row_size, 3);
        assert_eq!(m3.col_size, 3);
        assert_eq!(m3.data, matrix);
    }

    #[test]
    fn Matrix_mul_test() {
        let m1 = Matrix::new(3, 3, 2);
        let m2 = Matrix::new(3, 3, 3);
        let m3 = m1 * m2;
        let matrix = vec![vec![18, 18, 18], vec![18, 18, 18], vec![18, 18, 18]];

        assert_eq!(m3.data, matrix);

        let mut m5 = Matrix::new(2, 2, 5);
        m5.data[0][0] = 1;
        m5.data[0][1] = 6;
        m5.data[1][0] = 3;
        m5.data[1][1] = 8;

        let mut m6 = Matrix::new(2, 2, 8);
        m6.data[0][0] = 2;
        m6.data[0][1] = 2;
        m6.data[1][0] = 9;
        m6.data[1][1] = 7;

        let second_matrix = vec![vec![56, 44], vec![78, 62]];
        assert_eq!((m5 * m6).data, second_matrix);

        let mut this_one = Matrix::new(2, 3, 1);
        this_one.data[0][1] = 2;
        this_one.data[0][2] = 3;
        this_one.data[1][0] = 4;
        this_one.data[1][1] = 5;
        this_one.data[1][2] = 6;

        let mut another = Matrix::new(3, 1, 9);
        another.data[1][0] = 8;
        another.data[2][0] = 7;

        let third_matrix = vec![vec![46], vec![118]];
        assert_eq!((this_one * another).data, third_matrix);

        // [3 4 2] x [13 9 7 15 = [83 63 37 75] (83 = 3x13 + 4x8 + 2x6)
        //             8 7 4 6
        //             6 4 0 3]
        let first_dot = Matrix::new_from_vec(vec![vec![3, 4, 2]]);
        let second_dot =
            Matrix::new_from_vec(vec![vec![13, 9, 7, 15], vec![8, 7, 4, 6], vec![6, 4, 0, 3]]);
        let merged = vec![vec![83, 63, 37, 75]];

        assert_eq!((first_dot * second_dot).data, merged);
    }

    #[test]
    fn Matrix_scalar_mul_test() {
        let m1 = Matrix::new(3, 3, 2);
        let m2 = m1.scalar_mul(3);
        let matrix = vec![vec![6, 6, 6], vec![6, 6, 6], vec![6, 6, 6]];

        assert_eq!(m2.data, matrix);

        let m3 = Matrix::new(4, 2, 3.0);
        let m4 = m3.scalar_mul(0.5);
        let second_matrix = vec![
            vec![1.5, 1.5],
            vec![1.5, 1.5],
            vec![1.5, 1.5],
            vec![1.5, 1.5],
        ];

        assert_eq!(m4.data, second_matrix);
    }

    #[test]
    fn Matrix_transpose_test() {
        let m1 = Matrix::new_from_vec(vec![vec![1, 2, 3], vec![4, 5, 6]]);

        let m2 = Matrix::new_from_vec(vec![vec![1, 4], vec![2, 5], vec![3, 6]]);

        assert_eq!((m1.transpose()).data, m2.data);
    }

    #[test]
    fn Matrix_cauchy_test() {
        let gfRes = Gf2p::new(3, 0x0B);
        if gfRes.is_err() {
            println!("{}", gfRes.err().unwrap());
            return;
            //assert!(res.err().unwrap().to_string());
        }
        let gf = gfRes.unwrap();

        let mut m2 = Matrix::new(1,2, 0u8);
        let mut c1 = MatrixU8::new_cauchy_matrix(&gf, 2, 5);
        println!("{}", format!("{}", c1.unwrap()));
    }

}
