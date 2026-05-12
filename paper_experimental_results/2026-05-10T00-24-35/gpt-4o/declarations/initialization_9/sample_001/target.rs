fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() {
    enum Members {
        MemberOne,
        MemberTwo,
    }

    let nm = [
        "member one", // MemberOne
        "member two", // MemberTwo
    ];

    if !check(nm[Members::MemberOne as usize], "member one") {
        std::process::exit(1);
    }
    if !check(nm[Members::MemberTwo as usize], "member two") {
        std::process::exit(2);
    }

    {
        let p = &nm;
        if p[Members::MemberOne as usize].chars().nth(7) != Some('o') {
            std::process::exit(3);
        }
        if p[Members::MemberTwo as usize].chars().nth(7) != Some('t') {
            std::process::exit(4);
        }
    }
}