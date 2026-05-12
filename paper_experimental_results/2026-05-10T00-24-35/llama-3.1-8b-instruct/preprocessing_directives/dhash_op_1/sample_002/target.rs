fn main() {
    let p = format!("x # # y");
    if p.len()!= 7 {
        return 1;
    }
    if p.chars().nth(0).unwrap()!= 'x' {
        return 2;
    }
    if p.chars().nth(1).unwrap()!= ' ' {
        return 3;
    }
    if p.chars().nth(2).unwrap()!= '#' {
        return 4;
    }
    if p.chars().nth(3).unwrap()!= '#' {
        return 5;
    }
    if p.chars().nth(4).unwrap()!= ' ' {
        return 6;
    }
    if p.chars().nth(5).unwrap()!= 'y' {
        return 7;
    }
    if p.chars().nth(6).unwrap()!= '\0' {
        return 8;
    }
    return 0;
}