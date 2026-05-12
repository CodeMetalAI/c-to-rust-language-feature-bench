enum Member {
    One,
    Two,
}

fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() {
    let nm = [
        Member::Two => "member two",
        Member::One => "member one",
    ];

    if !check(nm[Member::One], "member one") {
        return std::process::exit(1);
    }
    if !check(nm[Member::Two], "member two") {
        return std::process::exit(2);
    }

    let p = &nm;
    if p[Member::One].chars().nth(7) != Some('o') {
        return std::process::exit(3);
    }
    if p[Member::Two].chars().nth(7) != Some('t') {
        return std::process::exit(4);
    }

    std::process::exit(0);
}