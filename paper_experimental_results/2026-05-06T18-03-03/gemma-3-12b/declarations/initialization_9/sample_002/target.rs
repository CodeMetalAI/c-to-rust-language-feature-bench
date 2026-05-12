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
        ("member one", "member two") => {
            ["member one", "member two"]
        }
        _ => panic!("Unexpected values in nm array"),
    };

    if !check(nm_refs[Member::One as usize], "member one") {
        return 1;
    }
    if !check(nm_refs[Member::Two as usize], "member two") {
        return 2;
    }

    if nm_refs[Member::One as usize].chars().nth(7) != Some('o') {
        return 3;
    }
    if nm_refs[Member::Two as usize].chars().nth(7) != Some('t') {
        return 4;
    }

    0
}