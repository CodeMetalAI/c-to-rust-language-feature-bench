fn main() {
    const X: i32 = 2;
    const fn f(a: i32) -> i32 {
        a + X
    }
    
    let v = f(10);
    let u = f(4); // (3, 4) evaluates to 4 in Rust, not a tuple
    let w = f(0);

    let i = [1, 23, 4, 5, 0];

    let c = ["hello", ""];

    if v != 12 {
        std::process::exit(1);
    }
    if u != 6 {
        std::process::exit(2);
    }
    if w != 2 {
        std::process::exit(3);
    }

    if i[0] != 1 {
        std::process::exit(4);
    }
    if i[1] != 23 {
        std::process::exit(5);
    }
    if i[2] != 4 {
        std::process::exit(6);
    }
    if i[3] != 5 {
        std::process::exit(7);
    }

    if c[0].as_bytes()[0] != b'h' {
        std::process::exit(8);
    }
    if c[0].as_bytes()[1] != b'e' {
        std::process::exit(9);
    }
    if c[0].as_bytes()[2] != b'l' {
        std::process::exit(10);
    }
    if c[0].as_bytes()[3] != b'l' {
        std::process::exit(11);
    }
    if c[0].as_bytes()[4] != b'o' {
        std::process::exit(12);
    }
    if c[0].as_bytes()[5] != b'\0' {
        std::process::exit(13);
    }

    if !c[1].is_empty() {
        std::process::exit(14);
    }
}