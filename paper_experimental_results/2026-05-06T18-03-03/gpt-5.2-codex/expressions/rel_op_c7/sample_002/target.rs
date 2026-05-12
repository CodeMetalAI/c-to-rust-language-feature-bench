use std::process::exit;

fn main() {
    let arr = [42];
    let p_index: usize = 0;

    if p_index != 0 {
        exit(1);
    }

    let p_end1 = p_index + 1;
    let p_end2 = arr.len();

    if p_end1 != p_end2 {
        exit(2);
    }

    if p_end1 == p_index {
        exit(3);
    }
}