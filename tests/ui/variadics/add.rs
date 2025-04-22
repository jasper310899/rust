//@ run-pass
//@ compile-flags: -Z treat-err-as-bug=1

//#![feature(min_variadics)]

fn main() {
    let a : (u8, u8, ...(f32, f32)) = (1,2,3.0,4.0);
}