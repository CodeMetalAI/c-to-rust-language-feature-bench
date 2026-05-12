enum Members {
    MemberOne,
    MemberTwo,
}

fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() -> i32 {
    let nm: [&str; 2] = [
        "member one", // member_one
        "member two", // member_two
    ];

    if !check(nm[Members::MemberOne as usize], "member one") {
        return 1;
    }
    if !check(nm[Members::MemberTwo as usize], "member two") {
        return 2;
    }

    {
        let p: [&str; 2] = nm;
        if p[Members::MemberOne as usize].chars().nth(7) != Some('o') {
            return 3;
        }
        if p[Members::MemberTwo as usize].chars().nth(7) != Some('t') {
            return 4;
        }
    }

    0
}