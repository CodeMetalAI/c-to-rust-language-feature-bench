fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() {
    #[repr(usize)]
    #[derive(Clone, Copy)]
    enum Members {
        One,
        Two,
    }

    const MEMBER_ONE: usize = Members::One as usize;
    const MEMBER_TWO: usize = Members::Two as usize;

    let nm: [&str; 2] = [
        "member one", // index 0
        "member two", // index 1
    ];

    if !check(nm[MEMBER_ONE], "member one") {
        std::process::exit(1);
    }
    if !check(nm[MEMBER_TWO], "member two") {
        std::process::exit(2);
    }

    {
        let p: &[&str; 2] = &nm;
        if p[MEMBER_ONE].chars().nth(7) != Some('o') {
            std::process::exit(3);
        }
        if p[MEMBER_TWO].chars().nth(7) != Some('t') {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}