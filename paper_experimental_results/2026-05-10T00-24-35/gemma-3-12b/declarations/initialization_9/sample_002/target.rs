#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Member {
    MemberOne,
    MemberTwo,
}

fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() {
    let nm = [
        ("member one", Member::MemberOne),
        ("member two", Member::MemberTwo),
    ];

    if !check(nm[Member::MemberOne.clone()].0, "member one") {
        return 1;
    }
    if !check(nm[Member::MemberTwo.clone()].0, "member two") {
        return 2;
    }

    {
        let p = &nm;
        if p[Member::MemberOne.clone()].0.chars().nth(7) != Some('o') {
            return 3;
        }
        if p[Member::MemberTwo.clone()].0.chars().nth(7) != Some('t') {
            return 4;
        }
    }

    0
}