const MEMBER_ONE: usize = 0;
const MEMBER_TWO: usize = 1;

fn check(a: &str, b: &str) -> bool {
    a.as_bytes()[0] == b.as_bytes()[0]
}

fn main() {
    let nm: [&str; 2] = ["member one", "member two"];

    if !check(nm[MEMBER_ONE], "member one") {
        std::process::exit(1);
    }
    if !check(nm[MEMBER_TWO], "member two") {
        std::process::exit(2);
    }

    {
        let p = &nm;
        if p[MEMBER_ONE].as_bytes()[7] != b'o' {
            std::process::exit(3);
        }
        if p[MEMBER_TWO].as_bytes()[7] != b't' {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}