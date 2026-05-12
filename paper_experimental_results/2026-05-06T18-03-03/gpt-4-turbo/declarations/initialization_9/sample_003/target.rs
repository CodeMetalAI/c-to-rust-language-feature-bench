fn main() {
    enum Member {
        MemberOne,
        MemberTwo,
    }

    let nm = [
        "member one", // MemberOne
        "member two", // MemberTwo
    ];

    fn check(a: &str, b: &str) -> bool {
        a.chars().next() == b.chars().next()
    }

    if !check(nm[Member::MemberOne as usize], "member one") {
        return std::process::exit(1);
    }
    if !check(nm[Member::MemberTwo as usize], "member two") {
        return std::process::exit(2);
    }

    let p = &nm;
    if p[Member::MemberOne as usize].chars().nth(7) != Some('o') {
        return std::process::exit(3);
    }
    if p[Member::MemberTwo as usize].chars().nth(7) != Some('t') {
        return std::process::exit(4);
    }
}