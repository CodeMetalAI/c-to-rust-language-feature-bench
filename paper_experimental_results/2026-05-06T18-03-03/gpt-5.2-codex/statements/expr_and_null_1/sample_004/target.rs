use std::process::exit;

fn p(t: &mut i32) -> i32 {
    *t += 1;
    0
}

fn main() {
    let mut x: i32 = 0;
    let _ = p(&mut x);
    if x == 1 {
        exit(0);
    } else {
        exit(1);
    }
}