fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() {
    const MEMBER_ONE: usize = 0;
    const MEMBER_TWO: usize = 1;

    let nm: [&str; 2] = [
        [MEMBER_TWO] => "member two",
        [MEMBER_ONE] => "member one",
    ];

    if !check(nm[MEMBER_ONE], "member one") {
        std::process::exit(1);
    }
    if !check(nm[MEMBER_TWO], "member two") {
        std::process::exit(2);
    }

    {
        let p = &nm;
        if p[MEMBER_ONE].chars().nth(7) != Some('o') {
            std::process::exit(3);
        }
        if p[MEMBER_TWO].chars().nth(7) != Some('t') {
            std::process::exit(4);
        }
    }
}