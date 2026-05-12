#[derive(Debug)]
enum Member {
    One,
    Two,
}

fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() {
    let nm = [
        "member one",
        "member two",
    ];

    if !check(nm[Member::One as usize], "member one") {
        return 1;
    }
    if !check(nm[Member::Two as usize], "member two") {
        return 2;
    }

    {
        let p = &nm;
        if p[Member::One as usize].chars().nth(7).unwrap() != 'o' {
            return 3;
        }
        if p[Member::Two as usize].chars().nth(7).unwrap() != 't' {
            return 4;
        }
    }

    0
}