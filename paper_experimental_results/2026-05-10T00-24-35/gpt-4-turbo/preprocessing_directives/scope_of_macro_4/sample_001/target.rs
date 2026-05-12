fn main() {
    // Variable declaration and initialization
    let x1: i32 = 7;
    let x2: &str = "hi";

    // Simulated preprocessor macros
    let fmt = "x1= %d, x2= %s";
    let inc = "vers2.h";
    let a = "hello";
    let b = "hello, world";

    // Check conditions
    if x1 != 7 {
        std::process::exit(1);
    }
    if x2.as_bytes()[0] != b'h' {
        std::process::exit(2);
    }
    if x2.as_bytes()[1] != b'i' {
        std::process::exit(3);
    }
    if x2.len() != 2 {
        std::process::exit(4);
    }

    // `fmt` checks
    let fmt_bytes = fmt.as_bytes();
    let fmt_expected = b"x1= %d, x2= %s";
    if fmt_bytes.len() != fmt_expected.len() {
        std::process::exit(5);
    }
    if fmt_bytes != fmt_expected {
        for (i, (f, e)) in fmt_bytes.iter().zip(fmt_expected.iter()).enumerate() {
            if f != e {
                std::process::exit(6 + i as i32);
            }
        }
    }

    // `inc` checks
    let inc_bytes = inc.as_bytes();
    let inc_expected = b"vers2.h";
    if inc_bytes.len() != inc_expected.len() {
        std::process::exit(21);
    }
    if inc_bytes != inc_expected {
        for (i, (f, e)) in inc_bytes.iter().zip(inc_expected.iter()).enumerate() {
            if f != e {
                std::process::exit(22 + i as i32);
            }
        }
    }

    // `a` checks
    let a_bytes = a.as_bytes();
    let a_expected = b"hello";
    if a_bytes.len() != a_expected.len() {
        std::process::exit(30);
    }
    if a_bytes != a_expected {
        for (i, (f, e)) in a_bytes.iter().zip(a_expected.iter()).enumerate() {
            if f != e {
                std::process::exit(31 + i as i32);
            }
        }
    }

    // `b` checks
    let b_bytes = b.as_bytes();
    let b_expected = b"hello, world";
    if b_bytes.len() != b_expected.len() {
        std::process::exit(37);
    }
    if b_bytes != b_expected {
        for (i, (f, e)) in b_bytes.iter().zip(b_expected.iter()).enumerate() {
            if f != e {
                std::process::exit(38 + i as i32);
            }
        }
    }
}