fn main() {
    let c: char;
    let i: i32;
    let l: i64;

    i = 100;

    l = (c = i as char) as i64;

    if c != i as char {
        return;
    }
    if l != c as i64 {
        return;
    }
    if l != (i as char) as i64 {
        return;
    }
}