fn main() {
    let x1 = 7;
    let x2 = "hi";
    let fmt = format!("x1= %d, x2= %s", 1, 2);
    let inc = format!("vers2.h");
    let a = format!("hello");
    let b = format!("hello, world");

    if x1!= 7 {
        return;
    }
    if x2.chars().next().unwrap()!= 'h' {
        return;
    }
    if x2.chars().nth(1).unwrap()!= 'i' {
        return;
    }
    if x2.chars().nth(2).is_some() {
        return;
    }

    if fmt.len()!= 15 {
        return;
    }
    if fmt.chars().nth(0).unwrap()!= 'x' {
        return;
    }
    if fmt.chars().nth(1).unwrap()!= '1' {
        return;
    }
    if fmt.chars().nth(2).unwrap()!= '=' {
        return;
    }
    if fmt.chars().nth(3).unwrap()!= ' ' {
        return;
    }
    if fmt.chars().nth(4).unwrap()!= '%' {
        return;
    }
    if fmt.chars().nth(5).unwrap()!= 'd' {
        return;
    }
    if fmt.chars().nth(6).unwrap()!= ',' {
        return;
    }
    if fmt.chars().nth(7).unwrap()!= ' ' {
        return;
    }
    if fmt.chars().nth(8).unwrap()!= 'x' {
        return;
    }
    if fmt.chars().nth(9).unwrap()!= '2' {
        return;
    }
    if fmt.chars().nth(10).unwrap()!= '=' {
        return;
    }
    if fmt.chars().nth(11).unwrap()!= ' ' {
        return;
    }
    if fmt.chars().nth(12).unwrap()!= '%' {
        return;
    }
    if fmt.chars().nth(13).unwrap()!= 's' {
        return;
    }
    if fmt.chars().nth(14).is_some() {
        return;
    }

    if inc.len()!= 8 {
        return;
    }
    if inc.chars().nth(0).unwrap()!= 'v' {
        return;
    }
    if inc.chars().nth(1).unwrap()!= 'e' {
        return;
    }
    if inc.chars().nth(2).unwrap()!= 'r' {
        return;
    }
    if inc.chars().nth(3).unwrap()!= 's' {
        return;
    }
    if inc.chars().nth(4).unwrap()!= '2' {
        return;
    }
    if inc.chars().nth(5).unwrap()!= '.' {
        return;
    }
    if inc.chars().nth(6).unwrap()!= 'h' {
        return;
    }
    if inc.chars().nth(7).is_some() {
        return;
    }

    if a.len()!= 5 {
        return;
    }
    if a.chars().nth(0).unwrap()!= 'h' {
        return;
    }
    if a.chars().nth(1).unwrap()!= 'e' {
        return;
    }
    if a.chars().nth(2).unwrap()!= 'l' {
        return;
    }
    if a.chars().nth(3).unwrap()!= 'l' {
        return;
    }
    if a.chars().nth(4).unwrap()!= 'o' {
        return;
    }

    if b.len()!= 12 {
        return;
    }
    if b.chars().nth(0).unwrap()!= 'h' {
        return;
    }
    if b.chars().nth(1).unwrap()!= 'e' {
        return;
    }
    if b.chars().nth(2).unwrap()!= 'l' {
        return;
    }
    if b.chars().nth(3).unwrap()!= 'l' {
        return;
    }
    if b.chars().nth(4).unwrap()!= 'o' {
        return;
    }
    if b.chars().nth(5).unwrap()!= ',' {
        return;
    }
    if b.chars().nth(6).unwrap()!= ' ' {
        return;
    }
    if b.chars().nth(7).unwrap()!= 'w' {
        return;
    }
    if b.chars().nth(8).unwrap()!= 'o' {
        return;
    }
    if b.chars().nth(9).unwrap()!= 'r' {
        return;
    }
    if b.chars().nth(10).unwrap()!= 'l' {
        return;
    }
    if b.chars().nth(11).unwrap()!= 'd' {
        return;
    }
}