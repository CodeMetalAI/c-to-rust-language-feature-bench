fn main() {
    let a = [10, 20, 30];

    if a.len()!= 3 {
        return 1;
    }
    if &a.len()!= 3 {
        return 2;
    }

    if a[0]!= 10 {
        return 3;
    }
    if a[1]!= 20 {
        return 4;
    }
    if a[2]!= 30 {
        return 5;
    }

    if std::mem::size_of::<[i32; 3]>()!= 3 * std::mem::size_of::<i32>() {
        return 6;
    }
    if std::mem::align_of::<[i32; 3]>() == std::mem::align_of::<i32>() {
        return 7;
    }

    let s = "abc";
    if s.len()!= 3 {
        return 8;
    }
    if s.chars().nth(0).unwrap()!= 'a' {
        return 9;
    }
    if s.chars().nth(1).unwrap()!= 'b' {
        return 10;
    }
    if s.chars().nth(2).unwrap()!= 'c' {
        return 11;
    }
    if s.chars().nth(3).unwrap()!= '\0' {
        return 12;
    }

    return 0;
}