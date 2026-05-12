fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() {
    let nm: [&str; 2] = [
        "member one",
        "member two",
    ];

    if !check(nm[0], "member one") {
        std::process::exit(1);
    }
    if !check(nm[1], "member two") {
        std::process::exit(2);
    }

    let p = &nm;
    if p[0].chars().nth(7) != Some('o') {
        std::process::exit(3);
    }
    if p[1].chars().nth(7) != Some('t') {
        std::process::exit(4);
    }
}