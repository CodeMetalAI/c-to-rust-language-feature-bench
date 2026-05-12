#[derive(Debug)]
enum MyEnum {
    MemberOne,
    MemberTwo,
}

fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() {
    let nm = [
        MyEnum::MemberTwo.to_string(),
        MyEnum::MemberOne.to_string(),
    ];

    if !check(&nm[MyEnum::MemberOne as usize], "member one") {
        return 1;
    }
    if !check(&nm[MyEnum::MemberTwo as usize], "member two") {
        return 2;
    }

    {
        let p = &nm;
        if p[MyEnum::MemberOne as usize].chars().nth(7) != Some('o') {
            return 3;
        }
        if p[MyEnum::MemberTwo as usize].chars().nth(7) != Some('t') {
            return 4;
        }
    }

    0
}