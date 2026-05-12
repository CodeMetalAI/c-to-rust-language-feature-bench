fn p(t: &mut i32) {
    *t += 1;
}

fn main() {
    let mut x: i32 = 0;
    p(&mut x);
    std::process::exit(if x == 1 { 0 } else { 1 });
}