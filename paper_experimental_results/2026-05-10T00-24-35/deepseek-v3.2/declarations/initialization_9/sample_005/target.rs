fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() {
    enum Member { One, Two }

    let nm: [&str; 2] = [
        Member::Two as usize => "member two",
        Member::One as usize => "member one",
    ];

    if !check(nm[Member::One as usize], "member one") {
        std::process::exit(1);
    }
    if !check(nm[Member::Two as usize], "member two") {
        std::process::exit(2);
    }

    {
        let p = &nm;
        if p[Member::One as usize].chars().nth(7) != Some('o') {
            std::process::exit(3);
        }
        if p[Member::Two as usize].chars().nth(7) != Some('t') {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}