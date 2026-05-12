fn main() {
    let a: [i32; 3] = [10, 20, 30];

    if a.len()!= 3 {
        panic!("a is not an array of length 3");
    }
    if &a.len()!= 3 {
        panic!("&a is not a reference to an array of length 3");
    }

    if a[0]!= 10 {
        panic!("a[0] is not 10");
    }
    if a[1]!= 20 {
        panic!("a[1] is not 20");
    }
    if a[2]!= 30 {
        panic!("a[2] is not 30");
    }

    if a.len() * std::mem::size_of::<i32>()!= 12 {
        panic!("sizeof a is not 12");
    }
    if std::mem::align_of::<[i32; 3]>()!= std::mem::align_of::<i32>() {
        panic!("alignof a is not equal to alignof i32");
    }

    let s = "abc";
    if s.len()!= 3 {
        panic!("s is not a string of length 3");
    }
    if s.chars().next().unwrap()!= 'a' {
        panic!("s[0] is not 'a'");
    }
    if s.chars().nth(1).unwrap()!= 'b' {
        panic!("s[1] is not 'b'");
    }
    if s.chars().nth(2).unwrap()!= 'c' {
        panic!("s[2] is not 'c'");
    }
    if s.chars().nth(3).unwrap_or_default()!= '\0' {
        panic!("s[3] is not '\\0'");
    }
}