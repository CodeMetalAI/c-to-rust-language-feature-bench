fn main() {
    let p = format!("x {} #{} # {} y", "", "");
    if p.len() != 7 {
        return;
    }
    if p.chars().next().unwrap() != 'x' {
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
    if p.chars().last().unwrap() != '\0' {
        return;
    }
    println!("0");
}