fn f() -> i32 {
    -1
}

fn main() {
    let c: i8 = f() as i8;
    
    let eq = if c as i32 == -1 { 1 } else { 0 };
    
    // In Rust, i8 is signed, so char_is_signed would be true
    // We need to simulate C's char behavior - in C, char can be signed or unsigned
    // depending on the platform. We need to check if (char)-1 < 0
    // Since we're using i8 (signed char), -1 as i8 is -1, which is < 0
    let char_is_signed = (-1i8) < 0;
    
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