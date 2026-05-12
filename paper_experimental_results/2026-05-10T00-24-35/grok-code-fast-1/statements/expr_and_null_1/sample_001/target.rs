fn p(t: &mut i32) -> i32 {
    *t += 1;
    0
}

fn main() -> i32 {
    let mut x: i32 = 0;
    p(&mut x);
    if x == 1 {
        0
    } else {
        1
    }
}