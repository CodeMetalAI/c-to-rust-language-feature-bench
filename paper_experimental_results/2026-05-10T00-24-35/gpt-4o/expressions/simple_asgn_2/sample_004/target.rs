fn main() {
    let c: char;
    let i: i32;
    let l: i64;

    i = 100;

    // In Rust, 'char' is a Unicode scalar value, so direct assignment like in
    // C/C++ isn't directly analogous. We'll use a u8 for similar behavior.
    let c_as_u8 = i as u8;
    c = c_as_u8 as char;

    l = c_as_u8 as i64;

    if c_as_u8 != i as u8 {
        std::process::exit(1);
    }
    if l != c_as_u8 as i64 {
        std::process::exit(2);
    }
    if l != i as u8 as i64 {
        std::process::exit(3);
    }
}