fn streq(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut i = 0;
    
    loop {
        if i >= a_bytes.len() || i >= b_bytes.len() {
            return i >= a_bytes.len() && i >= b_bytes.len();
        }
        
        if a_bytes[i] != b_bytes[i] {
            return false;
        }
        
        if a_bytes[i] == 0 {
            return true;
        }
        
        i += 1;
    }
}

fn check_name(got: &str, expect: &str) -> bool {
    streq(got, expect)
}

fn f() -> i32 {
    if !check_name("f", "f") {
        return -1;
    }
    0
}

fn g() -> i32 {
    if !check_name("g", "g") {
        return -2;
    }
    0
}

fn main() -> std::process::ExitCode {
    if !check_name("main", "main") {
        return std::process::ExitCode::from(1);
    }
    let result = f() + g();
    std::process::ExitCode::from(result as u8)
}