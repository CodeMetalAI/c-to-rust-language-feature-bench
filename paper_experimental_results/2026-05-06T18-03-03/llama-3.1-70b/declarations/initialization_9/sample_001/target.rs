fn main() {
    enum Member { MemberOne, MemberTwo };

    let nm = [
        Member::MemberTwo => "member two",
        Member::MemberOne => "member one",
    ];

    if nm[Member::MemberOne].chars().next()!= Some('m') {
        return;
    }
    if nm[Member::MemberTwo].chars().next()!= Some('m') {
        return;
    }

    let p = &nm;
    if p[Member::MemberOne].chars().nth(7)!= Some('o') {
        return;
    }
    if p[Member::MemberTwo].chars().nth(7)!= Some('t') {
        return;
    }
}