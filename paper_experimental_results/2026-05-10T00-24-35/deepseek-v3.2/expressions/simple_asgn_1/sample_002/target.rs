fn f() -> i32 {
    -1
}

fn main() {
    let c: i8;
    
    // In C, assigning an int to char performs a conversion.
    // In Rust, we need to explicitly cast.
    let assignment_result = f() as i8;
    c = assignment_result;
    
    // Compare with -1, but note: in C, char is promoted to int for comparison
    let eq = (c as i32 == -1);
    
    // Check if char is signed by checking if (char)-1 < 0
    // In Rust, i8 is always signed, but let's check the actual value
    let char_is_signed = (c < 0);
    
    if char_is_signed {
        if eq != true {
            std::process::exit(1);
        }
    } else {
        if eq != false {
            std::process::exit(2);
        }
    }
    
    std::process::exit(0);
}