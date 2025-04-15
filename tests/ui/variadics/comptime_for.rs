//@ run-pass
//@ compile-flags: -Z treat-err-as-bug=1

//#![feature(min_variadics)]

fn main() {
    let b : (Option<u8>, Option<u8>, Option<u8>)= for a in (1,2,3) {
        Some(a)
    };    
}