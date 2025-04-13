//@ run-pass
//@ -Z treat-err-as-bug=1

//#![feature(min_variadics)]

fn main() {
    let t1 = (1,2,3);
    let t2 = (3,4,5);

    let t3 = MyAdd::add(t1,t2);
    assert_eq!(t3, (1,2,3,3,4,5));
}

pub trait MyAdd<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

impl<T1: Tuple, T2: Tuple> MyAdd<T2> for T1 {
    type Output = (...T1, ...T2);

    fn add(self, rhs: T2) -> Self::Output {
        (...self, ...rhs)
    }
}