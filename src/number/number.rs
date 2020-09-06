use std::ops::*;

pub trait XNum: Add + Sub + Mul + Div {
    fn num_type() ->String;
    fn n_0() ->XNum;
    fn n_1() ->XNum;
    fn n_u8(n: u8) ->XNum;
}


impl XNum for u8 {
    fn num_type() ->String {
        return "u8".to_string();
    }

    fn n_0() ->XNum { 0u8 }

    fn n_1() ->XNum { 1u8 }

    fn n_u8(n: u8) ->XNum { n }
}

impl XNum2 for u8 {
    fn N0() ->XNum2 {
        return 0u8;
    }
}

impl XNum for u16 {
    fn num_type() ->String {
        return "u16".to_string();
    }
}

impl XNum for u32 {
    fn num_type() ->String {
        return "u32".to_string();
    }
}

impl XNum for u64 {
    fn num_type() ->String {
        return "u64".to_string();
    }
}

impl XNum for u128 {
    fn num_type() ->String {
        return "u128".to_string();
    }
}


impl XNum for i8 {
    fn num_type() ->String {
        return "i8".to_string();
    }
}

impl XNum for i16 {
    fn num_type() ->String {
        return "i16".to_string();
    }
}

impl XNum for i32 {
    fn num_type() ->String {
        return "i32".to_string();
    }
}

impl XNum for i64 {
    fn num_type() ->String {
        return "i64".to_string();
    }
}

impl XNum for i128 {
    fn num_type() ->String {
        return "i128".to_string();
    }
}

impl XNum for f32 {
    fn num_type() ->String {
        return "f32".to_string();
    }
}

impl XNum for f64 {
    fn num_type() ->String {
        return "f64".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_type() {
        println!("{}", u8::num_type());
    }
}

