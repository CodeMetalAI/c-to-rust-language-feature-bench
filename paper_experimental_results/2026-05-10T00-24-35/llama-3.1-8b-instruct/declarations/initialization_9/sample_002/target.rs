enum Enum {
    MemberOne,
    MemberTwo,
}

fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() -> i32 {
    let members = ["member two", "member one"];

    if !check(members[0], "member one") {
        1
    } else if !check(members[1], "member two") {
        2
    } else {
        let p = &members[..];
        if p[0][7] != 'o' {
            3
        } else if p[1][7] != 't' {
            4
        } else {
            0
        }
    }
}