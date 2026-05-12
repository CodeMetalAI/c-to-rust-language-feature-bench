enum Member {
    MemberOne,
    MemberTwo,
}

fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() -> i32 {
    let nm: [&str; 2] = [
        "member one",
        "member two",
    ];

    if !check(nm[Member::MemberOne as usize], "member one") {
        return 1;
    }
    if !check(nm[Member::MemberTwo as usize], "member two") {
        return 2;
    }

    {
        let p: &[&str] = &nm;
        if p[Member::MemberOne as usize].chars().nth(7) != Some('o') {
            return 3;
        }
        if p[Member::MemberTwo as usize].chars().nth(7) != Some('t') {
            return 4;
        }
    }

    0
}