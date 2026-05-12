fn f() -> i32 {
    -1
}

fn main() {
    let c: i8 = f() as i8;
    
    let eq = if c as i32 == -1 { 1 } else { 0 };
    
    // In Rust, i8 is signed, so char_is_signed would be true
    // We need to simulate C's char behavior - in C, char signedness is implementation-defined
    // Since we're using i8 (signed), char_is_signed is true
    let char_is_signed = ((-1i8) < 0);
    
    if char_is_signed {
        if eq != 1 {
            std::process::exit(1);
        }
    } else {
        if eq != 0 {
            std::process::exit(2);
        }
    }
    
    std::process::exit(0);
}