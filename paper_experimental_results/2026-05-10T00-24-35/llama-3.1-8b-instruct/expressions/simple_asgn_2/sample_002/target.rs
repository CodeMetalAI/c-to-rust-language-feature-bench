fn main() -> i32 {
    let mut c: char = '\0';
    let mut i: i32 = 100;
    let l: i64 = i as i64;

    l = c = i as u8 as char as i64;

    if c!= i as char {
        return 1;
    }
    if l!= i as i64 {
        return 2;
    }
    if l!= i as i64 {
        return 3;
    }

    0
}