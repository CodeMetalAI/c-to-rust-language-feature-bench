fn p(t: &mut i32) {
    *t += 1;
    0
}

fn main() {
    let mut x = 0;
    p(&mut x);
    assert_eq!(x, 1);
    std::process::exit(match x {
        1 => 0,
        _ => 1,
    });
}