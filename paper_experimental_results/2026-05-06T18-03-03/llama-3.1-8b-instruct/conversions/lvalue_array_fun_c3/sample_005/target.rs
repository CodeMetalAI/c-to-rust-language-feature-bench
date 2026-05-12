fn main() {
    let a = [10, 20, 30];

    if a.len()!= 3 {
        return;
    }
    if &a!= &a[..] {
        return;
    }

    if a[0]!= 10 {
        return;
    }
    if a[1]!= 20 {
        return;
    }
    if a[2]!= 30 {
        return;
    }

    if a.len() * std::mem::size_of::<i32>()!= 12 {
        return;
    }
    if std::mem::align_of::<i32>() == std::mem::align_of::<[i32; 3]>() {
        return;
    }

    let s = "abc";
    if s.len()!= 3 {
        return;
    }
    if s.chars().nth(0).unwrap()!= 'a' {
        return;
    }
    if s.chars().nth(1).unwrap()!= 'b' {
        return;
    }
    if s.chars().nth(2).unwrap()!= 'c' {
        return;
    }
    if s.chars().nth(3).unwrap()!= '\0' {
        return;
    }

    println!("All tests passed");
}