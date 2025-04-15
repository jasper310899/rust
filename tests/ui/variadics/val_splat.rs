//@ run-pass

//#![feature(min_variadics)]

fn main() {
    let a : (u8, u8, f32, f32) = (1,2, ...(3.0,4.0));
    assert_eq!(a, (1,2,3.0,4.0));

    let floats = (3.0,4.0);
    let a : (u8, u8, f32, f32) = (1,2, ...floats);
    assert_eq!(a, (1,2,3.0,4.0));

}