use std::process::exit;

const MEMBER_ONE: usize = 0;
const MEMBER_TWO: usize = 1;

fn check(a: &str, b: &str) -> bool {
    a.as_bytes()[0] == b.as_bytes()[0]
}

fn main() {
    let nm: [&str; 2] = [
        "member one", // MEMBER_ONE
        "member two", // MEMBER_TWO
    ];

    if !check(nm[MEMBER_ONE], "member one") {
        exit(1);
    }
    if !check(nm[MEMBER_TWO], "member two") {
        exit(2);
    }

    {
        let p = nm;
        if p[MEMBER_ONE].as_bytes()[7] != b'o' {
            exit(3);
        }
        if p[MEMBER_TWO].as_bytes()[7] != b't' {
            exit(4);
        }
    }

    exit(0);
}