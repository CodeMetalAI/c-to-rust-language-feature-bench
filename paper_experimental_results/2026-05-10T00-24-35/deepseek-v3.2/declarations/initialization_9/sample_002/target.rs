fn check(a: &str, b: &str) -> bool {
    a.chars().next() == b.chars().next()
}

fn main() {
    enum Members { One, Two, Count }
    
    let nm: [&str; Members::Count as usize] = [
        Members::Two as usize => "member two",
        Members::One as usize => "member one",
    ];
    
    if !check(nm[Members::One as usize], "member one") {
        std::process::exit(1);
    }
    if !check(nm[Members::Two as usize], "member two") {
        std::process::exit(2);
    }
    
    let p = &nm;
    if p[Members::One as usize].chars().nth(7) != Some('o') {
        std::process::exit(3);
    }
    if p[Members::Two as usize].chars().nth(7) != Some('t') {
        std::process::exit(4);
    }
    
    std::process::exit(0);
}