// Test Rust alignment support.  Test valid code using std::align.
// { dg-do run }
// { dg-options "-C" }

use std::mem::align_of;
use std::mem::align;

fn main() {
    // Test alignas
    let c: [u8; align_of::<max_align_t>()] = [0; align_of::<max_align_t>()];
    let c: [u8; align_of::<max_align_t>()] = [0; align_of::<max_align_t>()];
    let c: [u8; align_of::<max_align_t>()] = [0; align_of::<max_align_t>()];

    let s: [u16; align_of::<max_align_t>()] = [0; align_of::<max_align_t>()];

    let i: [u32; align_of::<max_align_t>()] = [0; align_of::<max_align_t>()];
    let i: [u32; align_of::<max_align_t>()] = [0; align_of::<max_align_t>()];

    let l: [u64; align_of::<max_align_t>()] = [0; align_of::<max_align_t>()];

    let ll: [u64; align_of::<max_align_t>()] = [0; align_of::<max_align_t>()];

    let f: [f32; align_of::<max_align_t>()] = [0.0; align_of::<max_align_t>()];

    let d: [f64; align_of::<max_align_t>()] = [0.0; align_of::<max_align_t>()];

    let cld: [std::complex::Complex<f64>; align_of::<max_align_t>()] = [std::complex::Complex::new(0.0, 0.0); align_of::<max_align_t>()];

    let ca: [u8; 10] = [0; 10];

    let x: [u32; align_of::<max_align_t>()] = [0; align_of::<max_align_t>()];

    enum E {
        E = align_of::<max_align_t>(),
    }

    let y: [u32; E::E] = [0; E::E];

    {
        let _auto_ll: [u64; align_of::<max_align_t>()] = [0; align_of::<max_align_t>()];
    }

    // Valid, but useless.
    struct S;

    // Test alignas and alignof macros
    const ALIGNAS: &str = "alignas";
    const ALIGNOF: &str = "alignof";
    const ALIGNAS_IS_DEFINED: &str = "__alignas_is_defined";
    const ALIGNOF_IS_DEFINED: &str = "__alignof_is_defined";

    assert_eq!(ALIGNAS, "alignas");
    assert_eq!(ALIGNOF, "alignof");
    assert_eq!(ALIGNAS_IS_DEFINED, "1");
    assert_eq!(ALIGNOF_IS_DEFINED, "1");
}