//@ run-pass
//@ compile-flags: -Z treat-err-as-bug=1

//#![feature(min_variadics)]

type Tu = (f32,f32,f32);
type Tu3 = (u8, &'static str);

fn main() {
    let _a : (u8, u8, ...(f32, f32, &'static str)) = (1,2,3.0,4.0, "test");
    assert_eq!(_a, (1,2,3.0,4.0, "test"));

     let c : (u8, ...Tu, ...Tu3) = (1, 2.0, 3.0, 4.0, 2, "test");
    assert_eq!(c, (1, 2.0, 3.0, 4.0, 2, "test"));
    
}