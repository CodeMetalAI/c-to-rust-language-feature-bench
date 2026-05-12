fn main() {
    enum Member { One, Two }

    let nm = [
        Member::Two => "member two",
        Member::One => "member one",
    ];

    if nm[Member::One].as_bytes()[0]!= b'm' {
        std::process::exit(1);
    }
    if nm[Member::Two].as_bytes()[0]!= b'm' {
        std::process::exit(2);
    }

    let p = &nm;
    if p[Member::One].as_bytes()[7]!= b'o' {
        std::process::exit(3);
    }
    if p[Member::Two].as_bytes()[7]!= b't' {
        std::process::exit(4);
    }
}