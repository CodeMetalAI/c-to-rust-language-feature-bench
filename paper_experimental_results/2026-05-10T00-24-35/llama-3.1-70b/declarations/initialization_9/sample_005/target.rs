fn main() {
    enum Member { MemberOne, MemberTwo }
    let nm = [
        Member::MemberTwo => "member two",
        Member::MemberOne => "member one",
    ];

    if nm[Member::MemberOne].chars().next().unwrap() != 'm' {
        return std::process::exit(1);
    }
    if nm[Member::MemberTwo].chars().next().unwrap() != 'm' {
        return std::process::exit(2);
    }

    let p = &nm;
    if p[Member::MemberOne].chars().nth(7).unwrap() != 'o' {
        return std::process::exit(3);
    }
    if p[Member::MemberTwo].chars().nth(7).unwrap() != 't' {
        return std::process::exit(4);
    }

    std::process::exit(0);
}