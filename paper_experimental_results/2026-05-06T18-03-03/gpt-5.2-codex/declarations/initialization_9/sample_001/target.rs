fn check(a: &str, b: &str) -> bool {
    a.as_bytes()[0] == b.as_bytes()[0]
}

fn main() {
    let member_one: usize = 0;
    let member_two: usize = 1;

    let mut nm = [""; 2];
    nm[member_two] = "member two";
    nm[member_one] = "member one";

    if !check(nm[member_one], "member one") {
        std::process::exit(1);
    }
    if !check(nm[member_two], "member two") {
        std::process::exit(2);
    }

    {
        let p = &nm;
        if p[member_one].as_bytes()[7] != b'o' {
            std::process::exit(3);
        }
        if p[member_two].as_bytes()[7] != b't' {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}