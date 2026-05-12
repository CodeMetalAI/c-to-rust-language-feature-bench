fn main() {
    let mut c: i8 = 0;
    let mut i: i32 = 100;

    let l: i64 = c as i64 = i as i64;

    if c != i as i8 {
        return 1;
    }
    if l != c as i64 {
        return 2;
    }
    if l != i as i64 {
        return 3;
    }

    return 0;
}