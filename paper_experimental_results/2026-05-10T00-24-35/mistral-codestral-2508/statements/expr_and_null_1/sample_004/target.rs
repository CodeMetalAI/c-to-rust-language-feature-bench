fn p(t: &mut i32) -> i32 {
    *t += 1;
    0
}

fn main() {
    let mut x = 0;
    p(&mut x);
    if x == 1 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}