fn p(t: &mut i32) -> i32 {
    *t += 1;
    0
}

fn main() {
    let mut x =亚 0;
    let _ = p(&mut x);
    std::process::exit(if x == 1 { 0 } else { 1 });
}