#[derive(Debug, Copy, Clone)]
enum Member {
    MemberOne,
    MemberTwo,
}

fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() {
    let nm = [
        [Member::MemberTwo, "member two"],
        [Member::MemberOne, "member one"],
    ];

    let nm_refs: [&str; 2] = [
        match nm[Member::MemberTwo as usize][1] {
            _ => nm[Member::MemberTwo as usize][1],
        },
        match nm[Member::MemberOne as usize][1] {
            _ => nm[Member::MemberOne as usize][1],
        },
    ];

    if !check(nm_refs[Member::MemberOne as usize], "member one") {
        return 1;
    }
    if !check(nm_refs[Member::MemberTwo as usize], "member two") {
        return 2;
    }

    {
        let p = &nm_refs;
        if p[Member::MemberOne as usize].chars().nth(7) != Some('o') {
            return 3;
        }
        if p[Member::MemberTwo as usize].chars().nth(7) != Some('t') {
            return 4;
        }
    }

    0
}