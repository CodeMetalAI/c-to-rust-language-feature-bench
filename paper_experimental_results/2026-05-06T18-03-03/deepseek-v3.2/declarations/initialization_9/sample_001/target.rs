fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() {
    enum Member {
        One,
        Two,
    }

    use Member::*;

    let nm = [
        Two => "member two",
        One => "member one",
    ];

    if !check(nm[One as usize], "member one") {
        std::process::exit(1);
    }
    if !check(nm[Two as usize], "member two") {
        std::process::exit(2);
    }

    {
        let p = &nm;
        if p[One as usize].as_bytes()[7] != b'o' {
            std::process::exit(3);
        }
        if p[Two as usize].as_bytes()[7] != b't' {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}