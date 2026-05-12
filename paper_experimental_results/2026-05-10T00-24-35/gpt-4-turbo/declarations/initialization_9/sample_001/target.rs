fn main() {
    let nm = [
        "member one", // member_one
        "member two", // member_two
    ];

    if !check(&nm[0], "member one") {
        std::process::exit(1);
    }
    if !check(&nm[1], "member two") {
        std::process::exit(2);
    }

    let p = &nm;
    if p[0].as_bytes()[7] != b'o' {
        std::process::exit(3);
    }
    if p[1].as_bytes()[7] != b't' {
        std::process::exit(4);
    }
}

fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}