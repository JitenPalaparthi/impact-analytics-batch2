fn main() {
    //    let result = obj.add(100).add(200).sub(100).mul(10).div(5).get();;
    //    fluent api, builder patter, chain of actions

    let mut data = Data::new(100);

    let r = data.add(100).sub(20).mul(3).div(2).get();
    println!("result:{}",r);
}

struct Data {
    d: i64,
}

impl Data {
    fn new(d: i64) -> Self {
        Self { d: d }
    }
}

trait Calc {
    fn add(&mut self, n: i64) -> &mut dyn Calc;
    fn sub(&mut self, n: i64) -> &mut dyn Calc;
    fn mul(&mut self, n: i64) -> &mut dyn Calc;
    fn div(&mut self, n: i64) -> &mut dyn Calc;
    fn get(&self) -> i64;
}

impl Calc for Data {
    fn add(&mut self, n: i64) -> &mut dyn Calc {
        self.d += n;
        return self;
    }
    fn sub(&mut self, n: i64) -> &mut dyn Calc {
        self.d -= n;
        return self;
    }

    fn mul(&mut self, n: i64) -> &mut dyn Calc {
        self.d *= n;
        return self;
    }

    fn div(&mut self, n: i64) -> &mut dyn Calc {
        self.d /= n;
        return self;
    }

    fn get(&self) -> i64 {
        return self.d;
    }
}
