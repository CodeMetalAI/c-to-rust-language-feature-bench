use std::process::exit;

fn main() {
    let arr = [42i32];
    let p: usize = 0;

    if p != 0 {
        exit(1);
    }

    let p_end1 = p + 1;
    let p_end2 = 0 + 1;

    if p_end1 != p_end2 {
        exit(2);
    }

    if p_end1 == p {
        exit(3);
    }

    exit(0);
}