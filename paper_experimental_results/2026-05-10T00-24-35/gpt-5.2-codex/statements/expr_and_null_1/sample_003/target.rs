use std::process::exit;

fn p(t: &mut i32) -> i32 {
    *t += 1;
    0
}

fn main() {
    let mut x = 0;
    let _ = p(&mut x);
    let code = if x == 1 { 0 } else { 1 };
    exit(code);
}