enum Member {
    MemberOne,
    MemberTwo,
}

fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() {
    let nm = [
        Member::MemberTwo => "member two",
        Member::MemberOne => "member one",
    ];

    if!check(nm[Member::MemberOne], "member one") {
        std::process::exit(1);
    }
    if!check(nm[Member::MemberTwo], "member two") {
        std::process::exit(2);
    }

    {
        let p = &nm;
        if p[Member::MemberOne].chars().nth(7).unwrap()!= 'o' {
            std::process::exit(3);
        }
        if p[Member::MemberTwo].chars().nth(7).unwrap()!= 't' {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}