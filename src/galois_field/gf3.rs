use rand::distributions::uniform::SampleUniform;

#[derive(Debug, Clone)]
pub struct Gf3<Num> {
    v: Vec<Num>
}

impl<Num> Gf3<Num>
   where Num: Default + Copy + SampleUniform,
{
    pub fn new(val: T) ->Self {
        let res = Gf{
            v: vec![val; 100],
        };
        return res;
    }

    pub fn get_head(&self) ->T {
        return *self.v.get(0).unwrap();
    }
}