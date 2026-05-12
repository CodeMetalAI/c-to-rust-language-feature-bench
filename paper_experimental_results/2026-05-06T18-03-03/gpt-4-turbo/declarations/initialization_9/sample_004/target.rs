fn main() {
    const MEMBER_ONE: usize = 0;
    const MEMBER_TWO: usize = 1;

    let nm = [
        "member one",
        "member two",
    ];

    if !check(nm[MEMBER_ONE], "member one") {
        return std::process::exit(1);
    }
    if !check(nm[MEMBER_TWO], "member two") {
        return std::process::exit(2);
    }

    let p = &nm;
    if p[MEMBER_ONE].as_bytes()[7] != b'o' {
        return std::process::exit(3);
    }
    if p[MEMBER_TWO].as_bytes()[7] != b't' {
        return std::process::exit(4);
    }

    return std::process::exit(0);
}

fn check(a: &str, b: &str) -> bool {
    a.as_bytes()[0] == b.as_bytes()[0]
}