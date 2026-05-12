fn main() {
    enum Member {
        One = 0,
        Two = 1,
    }

    let nm = [
        "member one", // Member::One
        "member two", // Member::Two
    ];

    if !check(nm[Member::One as usize], "member one") {
        return std::process::exit(1);
    }
    if !check(nm[Member::Two as usize], "member two") {
        return std::process::exit(2);
    }

    let p = &nm;
    if p[Member::One as usize].as_bytes()[7] != 'o' as u8 {
        return std::process::exit(3);
    }
    if p[Member::Two as usize].as_bytes()[7] != 't' as u8 {
        return std::process::exit(4);
    }

    return std::process::exit(0);
}

fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}