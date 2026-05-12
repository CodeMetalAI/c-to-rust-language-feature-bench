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
        Member::Two => "member two",
        Member::One => "member one",
    ];

    let nm_refs = match (nm[Member::One], nm[Member::Two]) {
        (a, b) => {
            [a, b]
        }
    };

    if !check(nm_refs[Member::One], "member one") {
        return 1;
    }
    if !check(nm_refs[Member::Two], "member two") {
        return 2;
    }

    if nm_refs[Member::One].chars().nth(7) != Some('o') {
        return 3;
    }
    if nm_refs[Member::Two].chars().nth(7) != Some('t') {
        return 4;
    }

    0
}