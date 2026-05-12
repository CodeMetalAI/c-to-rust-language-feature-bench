fn main() {
    enum Member {
        MemberOne,
        MemberTwo,
    }

    let nm = [
        Member::MemberTwo => "member two",
        Member::MemberOne => "member one",
    ];

    if nm[Member::MemberOne]!= "member one" {
        return 1;
    }
    if nm[Member::MemberTwo]!= "member two" {
        return 2;
    }

    {
        let p = &nm;
        if p[Member::MemberOne].chars().nth(7).unwrap()!= 'o' {
            return 3;
        }
        if p[Member::MemberTwo].chars().nth(7).unwrap()!= 't' {
            return 4;
        }
    }

    return 0;
}