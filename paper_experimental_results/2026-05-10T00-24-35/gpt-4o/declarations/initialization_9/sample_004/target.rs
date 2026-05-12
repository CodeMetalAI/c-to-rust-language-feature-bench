fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() {
    enum Members {
        MemberOne,
        MemberTwo,
    }

    let nm = [
        (Members::MemberTwo, "member two"),
        (Members::MemberOne, "member one"),
    ];

    let nm_map: std::collections::HashMap<_, _> = nm.iter().cloned().collect();

    if !check(nm_map[&Members::MemberOne], "member one") {
        std::process::exit(1);
    }
    if !check(nm_map[&Members::MemberTwo], "member two") {
        std::process::exit(2);
    }

    let p = &nm_map;
    if p[&Members::MemberOne].chars().nth(7) != Some('o') {
        std::process::exit(3);
    }
    if p[&Members::MemberTwo].chars().nth(7) != Some('t') {
        std::process::exit(4);
    }
}