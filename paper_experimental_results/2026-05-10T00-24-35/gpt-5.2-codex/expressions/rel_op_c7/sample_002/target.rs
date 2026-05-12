use std::process::exit;

fn main() {
    let x = 42;
    let _p = 0usize; // index representing &x in a single-element array

    if _p != 0 {
        exit(1);
    }

    let p_end1 = _p + 1;
    let p_end2 = 0usize + 1;

    if p_end1 != p_end2 {
        exit(2);
    }

    if p_end1 == _p {
        exit(3);
    }

    exit(0);
}