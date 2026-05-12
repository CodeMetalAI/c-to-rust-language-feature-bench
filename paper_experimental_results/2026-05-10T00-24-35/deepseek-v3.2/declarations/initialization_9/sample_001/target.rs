enum Members {
    MemberOne,
    MemberTwo,
}

fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() {
    let nm = [
        // Note: Rust arrays require consistent ordering, but we can assign by index
        [Members::MemberTwo, "member two"],
        [Members::MemberOne, "member one"],
    ]
    .iter()
    .fold([""; 2], |mut acc, &[member, value]| {
        acc[member as usize] = value;
        acc
    });

    if !check(nm[Members::MemberOne as usize], "member one") {
        return std::process::exit(1);
    }
    if !check(nm[Members::MemberTwo as usize], "member two") {
        return std::process::exit(2);
    }

    let p = &nm;
    if p[Members::MemberOne as usize].chars().nth(7) != Some('o') {
        return std::process::exit(3);
    }
    if p[Members::MemberTwo as usize].chars().nth(7) != Some('t') {
        return std::process::exit(4);
    }

    std::process::exit(0);
}