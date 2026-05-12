fn main() -> i32 {
    let mut c: char;
    let i: i32;
    let l: i64;

    i = 100;

    c = i as char;
    l = c as i64;

    if c != (i as char) {
        return 1;
    }
    if l != (c as i64) {
        return 2;
    }
    if l != (i as char) as i64 {
        return 3;
    }

    return 0;
}