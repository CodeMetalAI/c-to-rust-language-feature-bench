fn f() -> i32 {
    -1
}

fn main() {
    let c: i8 = f() as i8;
    
    let eq = if c as i32 == -1 { 1 } else { 0 };
    
    // In Rust, i8 is signed, so char_is_signed would be true
    // We need to simulate C's char behavior - in most systems char is signed
    let char_is_signed = ((-1i8) < 0) as i32;
    
    if char_is_signed != 0 {
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