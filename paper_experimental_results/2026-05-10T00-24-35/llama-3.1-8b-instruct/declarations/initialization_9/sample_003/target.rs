// initialization_9.rs
enum Member {
    MemberOne,
    MemberTwo,
}

fn check(a: &str, b: &str) -> bool {
    a.chars().next().unwrap() == b.chars().next().unwrap()
}

fn main() {
    let nm = [
        Member::MemberTwo => "member two",
        Member::MemberOne => "member one",
    ];

    if!check(nm.get(&Member::MemberOne).unwrap(), "member one") {
        return 1;
    }
    if!check(nm.get(&Member::MemberTwo).unwrap(), "member two") {
        return 2;
    }

    {
        let p = nm.get(&Member::MemberOne).unwrap();
        if p.chars().nth(7).unwrap()!= 'o' {
            return 3;
        }
        let p = nm.get(&Member::MemberTwo).unwrap();
        if p.chars().nth(7).unwrap()!= 't' {
            return 4;
        }
    }

    return 0;
}