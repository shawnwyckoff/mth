// Galois field arithmetic.

/*
another multiply algorithm on GF(2^8)： https://blog.csdn.net/codebreakers/article/details/41456149?locationNum=7&fps=1
*/

use std::collections::HashMap;

// TODO
// 不可约多项式改用u128
// 支持GF(2)

const DEFAULT_IRREDUCIBLE_POLYNOMIAL_DICT: [u8; 9] = [
    0x00, // GF(2^0) not exist
    0x03, // 0b11: x + 1
    0x07, // 0b111: x^2 + x + 1
    0x0B, // 0b001011: x^3 + x + 1
    0x13, // 0b010011: x^4 + x + 1
    0x25, // 0b100101: x^5 + x^2 + 1
    0x43, // 0b001000011: x^6 + x + 1
    0x83, // 0b010000011: x^7 + x + 1
    0x1D, // 0b100011101 % 0xFF = 0x1D : x^8 + x^4 + x^3 + x^2 + 1
];

/**
in AES(Rijndael), the irreducible polynomial is x⁸ + x⁴ + x³ + x + 1，this polynomial = 0x11B mod 0xFF = 0x1B when x = 2
*/
const GF2P8_IRREDUCIBLE_POLYNOMIAL_AES: u8 = 0x1B;

/**
in Erasure Code, the irreducible polynomial is x⁸ + x⁴ + x³ + x² + 1，this polynomial = 0x1D when x = 2
*/
const GF2P8_IRREDUCIBLE_POLYNOMIAL_ERASURE_CODE: u8 = 0x1D;

/**
Galois field arithmetic on $GF(2^w)$
*/
pub struct Gf2p {
    // notice: this is how many elements exist on current finite field,
    // but this number is less than power table length, element_count == power_length - 1.
    element_count: usize,
    overflow_flag: u8,
    w: u8,
    irreducible_polynomial: u8,
    power: Vec<u8>, // ilog table
    log: Vec<u8>,   // log table
}

impl Gf2p {
    /**
    Create new galois field calculator.
    w: power.
    irreducible_polynomial: irreducible polynomial selected, and sometimes it is primitive polynomial, sometimes it is not.
    */
    pub fn new(w: u8, irreducible_polynomial: u8) -> anyhow::Result<Gf2p> {
        if w <= 0 || w >= 9 {
            return Err(anyhow::anyhow!(
                "w cannot be ".to_string() + w.to_string().as_str()
            ));
        }

        let element_count: usize = 1 << (w as usize);
        let mut gf = Gf2p {
            element_count: 1 << w as usize,
            overflow_flag: 1 << (w - 1), // on GF(2^8) it is 0x80
            w: w,
            irreducible_polynomial: irreducible_polynomial,
            power: vec![0; element_count],
            log: vec![0; element_count],
        };

        let res = gf.generate_power_log_table();
        if res.is_err() {
            return Err(res.unwrap_err());
        }
        return Ok(gf);
    }

    /**
    Generate power table and log table lookup table calculator for $GF(2^w)$.
    g: generator, on $GF(2^w)$, 2 is NOT always generator, it depends on what irreducible polynomial selected.
    power table: table to store g⁰, g¹, g² ... g^(2^w), element pᵢ = g^i.
    log table: table to store log_g{0}, log_g{1}, log_g{2} ... log_g{2^w}, element lᵢ = log_g{i}
    */
    fn generate_power_log_table(&mut self) -> Result<(), anyhow::Error> {
        let mut n = 1u8;
        let g = self.min_generator();
        if g.is_none() {
            return Err(anyhow::anyhow!(
                "generator not found for w ".to_string() + self.w.to_string().as_str()
            ));
        }

        self.power[0] = 1; // g(0) = 1, first element is 1, it is used to generate power table

        for i in 1u8..=((self.element_count - 1) as u8) {
            // g(i) = g(i - 1) * g
            // notice, power table is generated by multiply generator, generator is 2 sometimes, but NOT 2 all times. So, mul generator, NOT 2!
            n = self.mul(n, g.unwrap());

            self.power[i as usize] = n; //self.power.get_mut(i as usize).map(|item| *item = n);
            self.log[n as usize] = i; //self.log.get_mut(n as usize).map(|item| *item = i);
        }

        // power[0] = 1, so log[1] = 0, this operation should be put after for cycle, because is could be input with log[0xFF] = 0 in for cycle.
        // power table start with 1 but not 0, so power table has two 0x01 and lack of 0x00.
        // log table is the inverse operation of power table, so log table has two 0x00 and lack of 0xFF.
        self.log[1] = 0;
        return Ok(());
    }

    pub fn min_element(&self) -> u8 {
        return 0;
    }

    pub fn max_element(&self) -> u8 {
        return ((1u16 << (self.w as u16)) as u8) - 1;
    }

    // How many element exist on current field.
    pub fn get_element_count(&self) -> usize {
        return self.element_count;
    }

    pub fn power_table(self) -> Vec<u8> {
        return self.power;
    }

    pub fn log_table(self) -> Vec<u8> {
        return self.log;
    }

    /**
    add on $GF(2^w)$
    on $GF(2^w)$, x + y is x xor y
    */
    #[inline(always)]
    pub fn add(&self, x: u8, y: u8) -> u8 {
        x ^ y
    }

    /**
    FIXME 有些文章说减法就是加上加法逆元，和加法并不相同！
    sub on $GF(2^w)$
    on $GF(2^w)$, x - y is x xor y, same as add
    */
    #[inline(always)]
    pub fn sub(&self, x: u8, y: u8) -> u8 {
        x ^ y
    }

    /**
    x multiply 2 on $GF(2^w)$
    */
    #[inline(always)]
    fn mul_2(&self, x: u8) -> u8 {
        // overflow_flag is 0x80 on $GF(2^8)$
        // x << 1 means multiply 2
        //
        // if x & self.overflow_flag != 0, means highest bit is 1, number will overflow after multiply 2 (left shift 1)
        // if overflows, then mod irreducible polynomial, in GF(2^w), mod irreducible polynomial == XOR
        if (x & self.overflow_flag) != 0 {
            (x << 1) ^ self.irreducible_polynomial
        } else {
            x << 1
        }
    }

    /**
    Multiplication direcly on $GF(2^w)$, this algorithm comes from my mathematics notes.
    x, y: input numbers
    */
    #[inline(always)]
    pub fn mul(&self, x: u8, y: u8) -> u8 {
        // temp table which stores x mul 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80
        let mut x_mul_0x01_0x80: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        x_mul_0x01_0x80[0] = x;
        for i in 1..8 {
            x_mul_0x01_0x80[i] = self.mul_2(x_mul_0x01_0x80[i - 1]);
        }

        let mut res: u8 = 0;
        for i in 0..8 {
            // if ((y >> i) & 0x01) == 0, nothing changed in res
            res ^= ((y >> i) & 0x01) /*1: yes, 0: no*/ * x_mul_0x01_0x80[i];
        }
        return res;
    }

    /**
    Multiplication on $GF(2^w)$ through looking up power table and log table.

    if x =0 or y = 0
    $$
    x * y = 0
    $$

    if x != 0 and y != 0

    $$
    x * y = g^{(log_g{x}+log_g{y}) \mod (2^w - 1)}
    $$

    In this formula, "+" is add like usually, not "^". If sum overflows, mod $(2^w - 1)$, it is 0xFF(255) on $GF(2^8)$
    */
    #[inline(always)]
    pub fn mul_by_power_log_table(&self, x: u8, y: u8) -> u8 {
        // in power table there is no 0, if input number has 0, just return correct result -> 0.
        if x == 0 || y == 0 {
            return 0;
        }

        // if (log_table[x] + log_table[y]) overflows, mod (element - 1), it is 0xFF(255) on $GF(2^8)$
        let mut sum: u16 = ((self.log[x as usize] as u16) + (self.log[y as usize] as u16))
            % (self.element_count - 1) as u16;
        return self.power[sum as usize];
    }

    /**
    Division on $GF(2^w)$ through looking up power table and log table.
        $$
        x / y = g^{(log_g{x}-log_g{y}) \mod (2^w - 1)}
        $$
    */
    #[inline(always)]
    pub fn div_by_power_log_table(&self, x: u8, y: u8) -> u8 {
        if x == 0 {
            return 0u8;
        } // 0 mul any number equals 0
        assert_ne!(y, 0); // can't divide 0

        let mut difference_i64 = (self.log[x as usize] as i64) - (self.log[y as usize] as i64);
        if difference_i64 < 0 {
            difference_i64 += (self.element_count - 1) as i64;
        }
        let mut difference: usize = (difference_i64 as usize) % (self.element_count - 1) as usize;
        return self.power[difference];
    }

    /**
    Multiplication on $GF(2^w)$ utilising SIMD through looking up half table.
    */
    #[inline(always)]
    pub fn mul_by_simd() -> u8 {
        0
    }

    /**
    Check input number is generator or not.
    */
    fn is_generator(&self, generator: u8) -> bool {
        let mut products: HashMap<u8, bool> = HashMap::new();
        let mut n = generator;
        for i in 1u8..=(self.element_count - 1) as u8 {
            // found duplicated product, it means this is not a product
            if products.contains_key(&n) {
                return false;
            }

            // save product to map cache
            products.insert(n, true);

            // calculate product
            n = self.mul(n, generator);
        }

        // check products are complete
        if products.len() != (self.element_count as usize - 1) {
            return false;
        }

        // in power table, there are 2 duplicated element '1', so check from 2
        for n in 2u8..=(self.element_count - 1) as u8 {
            if !products.contains_key(&n) {
                return false;
            }
        }

        return true;
    }

    /**
    Find all generators of $GF(2^w)$
    */
    pub fn min_generator(&self) -> Option<u8> {
        for g in 2u8..=((self.element_count - 1) as u8) {
            if self.is_generator(g) {
                return Some(g);
            }
        }
        return None;
    }

    /**
    Find all generators of $GF(2^w)$
    */
    pub fn all_generators(&self) -> Vec<u8> {
        let mut res = vec![];
        for g in 2u8..=((self.element_count - 1) as u8) {
            if self.is_generator(g) {
                res.push(g);
            }
        }
        return res;
    }
}

pub fn print_matrix_u8(data: Vec<u8>) {
    let side_len = (data.len() as f64).sqrt() as usize;
    for (k, v) in data.iter().enumerate() {
        if (k + 1) % side_len == 0 {
            // last element
            println!("{:02X}", v);
        } else {
            // non-last element
            print!("{:02X},", v);
        }
    }
    println!();
}

pub fn print_matrix_u16(data: Vec<u16>) {
    let side_len = (data.len() as f64).sqrt() as usize;
    for (k, v) in data.iter().enumerate() {
        if (k + 1) % side_len == 0 {
            // last element
            println!("{:02X}", v);
        } else {
            // non-last element
            print!("{:02X},", v);
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Gf2p_generate_power_log_table_test() {
        let res = Gf2p::new(3, 0x0B);
        if res.is_err() {
            println!("{}", res.err().unwrap());
            return;
            //assert!(res.err().unwrap().to_string());
        }
        //assert_eq!(res.is_ok(), true);
        let gf = res.unwrap();

        //println!("{}", gf.power.to_string());

        print_matrix_u8(gf.power);

        print_matrix_u8(gf.log);
    }

    #[test]
    fn Gf2p_mul_test() {
        let res = Gf2p::new(8, 0x1B);
        if res.is_err() {
            println!("{}", res.err().unwrap());
            return;
            //assert!(res.err().unwrap().to_string());
        }
        //assert_eq!(res.is_ok(), true);
        let dt = res.unwrap();
    }

    #[test]
    fn Gf2p_add_test() {
        let mut res = Gf2p::new(3, 0x0B);
        if res.is_err() {
            println!("{}", res.err().unwrap());
            return;
            //assert!(res.err().unwrap().to_string());
        }
        let gf = res.unwrap();

        for x in 0x00u8..=7 {
            for y in 0x00u8..=7 {
                let quotient = gf.add(x, y);
                println!("{} + {} = {}", x, y, quotient);
            }
        }
    }

    #[test]
    fn Gf2p_div_by_power_log_table_test() {
        let mut res = Gf2p::new(3, 0x0B);
        if res.is_err() {
            println!("{}", res.err().unwrap());
            return;
            //assert!(res.err().unwrap().to_string());
        }
        let gf = res.unwrap();

        for x in 0x00u8..=7 {
            for y in 0x01u8..=7 {
                let quotient = gf.div_by_power_log_table(x, y);
                println!("{} / {} = {}", x, y, quotient);
            }
        }
    }

    #[test]
    fn Gf2p_diff_mul_cmp_test() {
        for w in 1u8..=8 {
            let mut res = Gf2p::new(w, DEFAULT_IRREDUCIBLE_POLYNOMIAL_DICT[w as usize]);
            if res.is_err() {
                println!("{}", res.err().unwrap());
                return;
                //assert!(res.err().unwrap().to_string());
            }
            let gf = res.unwrap();

            let maxN = ((1 << w as u16) - 1) as u8; // if w = 8, maxN = 0xFF

            for x in 0x00u8..=maxN {
                for y in 0x00u8..=maxN {
                    let sum_direct = gf.mul(x, y);
                    let sum_table = gf.mul_by_power_log_table(x, y);
                    if sum_direct != sum_table {
                        println!(
                            "x: {}, y: {}: sum_direct: {} vs sum_table: {}",
                            x, y, sum_direct, sum_table
                        );
                    }
                    assert_eq!(sum_direct, sum_table);
                }
            }
        }
    }

    #[test]
    fn Gf2p_all_generators_test() {
        let mut res = Gf2p::new(8, 0x1B);
        if res.is_err() {
            println!("{}", res.err().unwrap());
            return;
            //assert!(res.err().unwrap().to_string());
        }
        let gf = res.unwrap();
        let allGeneratos = gf.all_generators();
        let mut allGeneratosString = "[".to_string();
        for (k, v) in allGeneratos.iter().enumerate() {
            allGeneratosString += v.to_string().as_str();
            if k != allGeneratos.len() - 1 {
                allGeneratosString += ",";
            }
        }
        allGeneratosString += "]";

        // This generators list is from GF(2^8) in AES algorithm, irreducible polynomial is 0x1B (x⁸ + x⁴ + x³ + x + 1).
        assert_eq!(allGeneratosString, "[3,5,6,9,11,14,17,18,19,20,23,24,25,26,28,30,31,33,34,35,39,\
        40,42,44,48,49,60,62,63,65,69,70,71,72,73,75,76,78,79,82,84,86,87,88,89,90,91,95,100,101,104,\
        105,109,110,112,113,118,119,121,122,123,126,129,132,134,135,136,138,142,143,144,147,149,150,152,\
        153,155,157,160,164,165,166,167,169,170,172,173,178,180,183,184,185,186,190,191,192,193,196,200,201,\
        206,207,208,214,215,218,220,221,222,226,227,229,230,231,233,234,235,238,240,241,244,245,246,248,251,253,254,255]");
    }
}
