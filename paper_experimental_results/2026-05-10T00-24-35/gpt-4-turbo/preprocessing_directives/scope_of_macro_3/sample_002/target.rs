fn main() {
    let v = f(10);
    let u = f((3, 4).1); // translates the use of comma operator in macro argument
    let w = t(g())(0);

    let i = [1, 23, 4, 5, 0];
    let c = ["hello".to_string(), "".to_string()];

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

    let c_bytes = c.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();

    if c_bytes[0][0] != b'h' {
        std::process::exit(8);
    }
    if c_bytes[0][1] != b'e' {
        std::process::exit(9);
    }
    if c_bytes[0][2] != b'l' {
        std::process::exit(10);
    }
    if c_bytes[0][3] != b'l' {
        std::process::exit(11);
    }
    if c_bytes[0][4] != b'o' {
        std::process::exit(12);
    }
    if c_bytes[0].get(5) != Some(&b'\0') {
        std::process::exit(13);
    }

    if !c_bytes[1].is_empty() {
        std::process::exit(14);
    }
}

fn f(a: i32) -> i32 {
    a + 2
}

fn t<F>(f: F) -> F
where
    F: Fn(i32) -> i32,
{
    f
}

fn g() -> fn(i32) -> i32 {
    f
}