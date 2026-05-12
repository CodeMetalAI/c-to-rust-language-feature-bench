const MEMBER_ONE: usize = 0;
const MEMBER_TWO: usize = 1;

fn check(a: &str, b: &str) -> bool {
    a.as_bytes()[0] == b.as_bytes()[0]
}

fn main() -> i32 {
    let nm: [&str; 2] = [
        "member one",
        "member two",
    ];

    if !check(nm[MEMBER_ONE], "member one") {
        return 1;
    }
    if !check(nm[MEMBER_TWO], "member two") {
        return 2;
    }

    {
        let p = &nm;
        if p[MEMBER_ONE].as_bytes()[7] != b'o' {
            return 3;
        }
        if p[MEMBER_TWO].as_bytes()[7] != b't' {
            return 4;
        }
    }

    0
}