fn main() {
    let mut c: i8;
    let mut i: i32;
    let mut l: i64;

    i = 100;

    l = {
        c = i as i8;
        c
    } as i64;

    if c != (i as i8) {
        return;
    }
    if l != (c as i64) {
        return;
    }
    if l != (i as i8) as i64 {
        return;
    }

    println!("Success");
}