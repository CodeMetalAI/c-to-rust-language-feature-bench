fn main() {
    enum Member {
        One = 0,
        Two = 1,
    }

    let nm = [
        "member one",
        "member two",
    ];

    if !check(nm[Member::One as usize], "member one") {
        std::process::exit(1);
    }
    if !check(nm[Member::Two as usize], "member two") {
        std::process::exit(2);
    }

    let p = &nm;
    if p[Member::One as usize].as_bytes()[7] != b'o' {
        std::process::exit(3);
    }
    if p[Member::Two as usize].as_bytes()[7] != b't' {
        std::process::exit(4);
    }

    std::process::exit(0);
}

fn check(a: &str, b: &str) -> bool {
    a.as_bytes()[0] == b.as_bytes()[0]
}