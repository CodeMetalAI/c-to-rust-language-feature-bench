enum Member {
    MemberOne,
    MemberTwo,
}

fn check(a: &str, b: &str) -> bool {
    a.chars().next().unwrap() == b.chars().next().unwrap()
}

fn main() {
    let nm = [
        (Member::MemberTwo, "member two"),
        (Member::MemberOne, "member one"),
    ];

    if!check(&nm[Member::MemberOne].1, "member one") {
        std::process::exit(1);
    }
    if!check(&nm[Member::MemberTwo].1, "member two") {
        std::process::exit(2);
    }

    {
        let p = &nm;
        if p[Member::MemberOne].1.chars().nth(7).unwrap()!= 'o' {
            std::process::exit(3);
        }
        if p[Member::MemberTwo].1.chars().nth(7).unwrap()!= 't' {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}