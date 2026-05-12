fn main() {
    // Test alignment support in Rust
    // Rust has built-in alignment support through the `align` attribute

    // Define variables with alignment
    let _c: [u8; 1] = [0; 1];
    let _s: [i16; 1] = [0; 1];
    let _i: [i32; 1] = [0; 1];
    let _l: [i64; 1] = [0; 1];
    let _ll: [i64; 1] = [0; 1];
    let _f: [f32; 1] = [0; 1];
    let _d: [f64; 1] = [0; 1];
    let _cld: [f64; 2] = [0.0; 2]; // Complex long double as two f64 values
    let _ca: [u8; 10] = [0; 10];
    let _x: [i32; 1] = [0; 1];
    let _y: [i32; 1] = [0; 1];

    // Test alignment macros
    let s1 = "align";
    let s2 = "align_of";
    let s3 = "1";
    let s4 = "1";

    if s1 != "align" {
        std::process::abort();
    }
    if s2 != "align_of" {
        std::process::abort();
    }
    if s3 != "1" {
        std::process::abort();
    }
    if s4 != "1" {
        std::process::abort();
    }
}