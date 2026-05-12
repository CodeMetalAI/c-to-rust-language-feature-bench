fn main() {
    let p = format!("x {} #{} y", 0, 0);
    if p.len() != 7 {
        return;
    }
    if p.chars().nth(0).unwrap() != 'x' {
        return;
    }
    if p.chars().nth(1).unwrap() != ' ' {
        return;
    }
    if p.chars().nth(2).unwrap() != '#' {
        return;
    }
    if p.chars().nth(3).unwrap() != '#' {
        return;
    }
    if p.chars().nth(4).unwrap() != ' ' {
        return;
    }
    if p.chars().nth(5).unwrap() != 'y' {
        return;
    }
    if p.chars().nth(6).unwrap() != '\0' {
        return;
    }
    println!("0");
}