fn main() -> i32 {
    let i: i32 = 100;
    let c: char = i as char;
    let l: i64 = c as i64;

    if c != (i as char) {
        return 1;
    }
    if l != (c as i64) {
        return 2;
    }
    if l != (i as char) as i64 {
        return 3;
    }

    0
}