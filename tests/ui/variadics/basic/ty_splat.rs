//@ run-pass
//@ compile-flags: -Z treat-err-as-bug=1

//#![feature(min_variadics)]

type Tu = (f32,f32,f32);
type Tu3 = (u8, &'static str);

const _SPLAT_CONST : (u8, u8, ...(f32, f32, &'static str)) = (1,2,3.0,4.0, "test");
static _SPLAT_STATIC : (u8, u8, ...(f32, f32, &'static str)) = (1,2,3.0,4.0, "test");


fn main() {
    let _a : (u8, u8, ...(f32, f32, &'static str)) = (1,2,3.0,4.0, "test");
    assert_eq!(_a, (1,2,3.0,4.0, "test"));

    let c : (u8, ...Tu, ...Tu3) = (1, 2.0, 3.0, 4.0, 2, "test");
    assert_eq!(c, (1, 2.0, 3.0, 4.0, 2, "test"));
    
    let d : (f32, ...[u8; 2]) = (1.0, 2, 3);
    assert_eq!(d, (1.0, 2, 3));

    let mut e : (...[u8; 10],) = Default::default();
    e.5 = 2;
    assert_eq!(e, (0,0,0,0,0,2,0,0,0,0));

    let f : (u8, ...(u8, u8, ...(u8,u8))) = (1,2,3,4,5);
    assert_eq!(f, (1,2,3,4,5));
}