fn main() {
    let mut a: i32;
    let mut b: i32;
    let mut c: i32;
    let mut d: i32;

    // First check
    a = 1;
    b = 2;
    c = 3;
    d = 4;
    let left = if a < b { 1 } else { 0 };
    let mid = if left == c { 1 } else { 0 };
    let val = if mid < d { 1 } else { 0 };
    if val != 1 {
        std::process::exit(1);
    }

    // Second check
    a = 2;
    b = 1;
    c = 4;
    d = 3;
    let left = if a < b { 1 } else { 0 };
    let mid = if left == c { 1 } else { 0 };
    let val = if mid < d { 1 } else { 0 };
    if val != 1 {
        std::process::exit(2);
    }

    // Third check
    a = 1;
    b = 2;
    c = 4;
    d = 3;
    let left = if a < b { 1 } else { 0 };
    let mid = if left == c { 1 } else { 0 };
    let val = if mid < d { 1 } else { 0 };
    if val != 0 {
        std::process::exit(3);
    }

    // Fourth check
    a = 2;
    b = 1;
    c = 3;
    d = 4;
    let left = if a < b { 1 } else { 0 };
    let mid = if left == c { 1 } else { 0 };
    let val = if mid < d { 1 } else { 0 };
    if val != 0 {
        std::process::exit(4);
    }

    // Fifth check
    a = 1;
    b = 2;
    c = 3;
    d = 4;
    let val = if (a < b) == (c < d) { 1 } else { 0 };
    if val != 1 {
        std::process::exit(5);
    }

    std::process::exit(0);
}