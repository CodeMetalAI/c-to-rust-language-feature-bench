use std::process::exit;

fn main() {
    let arr = [42i32];

    let p = arr.as_ptr();

    if p != arr.as_ptr() {
        exit(1);
    }

    let range = arr.as_ptr_range();
    let p_end1 = range.end;
    let p_end2 = range.end;

    if p_end1 != p_end2 {
        exit(2);
    }

    if p_end1 == p {
        exit(3);
    }

    exit(0);
}