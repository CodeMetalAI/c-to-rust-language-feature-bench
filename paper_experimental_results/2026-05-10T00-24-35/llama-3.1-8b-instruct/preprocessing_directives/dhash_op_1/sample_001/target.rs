fn main() {
    let p = format!("x # # y");

    if p.len() != 7 {
        return 1;
    }

    if p.chars().nth(0) != Some('x') {
        return 2;
    }

    if p.chars().nth(1) != Some(' ') {
        return 3;
    }

    if p.chars().nth(2) != Some('#') {
        return 4;
    }

    if p.chars().nth(3) != Some('#') {
        return 5;
    }

    if p.chars().nth(4) != Some(' ') {
        return 6;
    }

    if p.chars().nth(5) != Some('y') {
        return 7;
    }

    if p.chars().nth(6) != Some('\0') {
        return 8;
    }

    return 0;
}