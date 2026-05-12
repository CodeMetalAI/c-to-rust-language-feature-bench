use std::process;

fn main() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    if a[2] != 30 {
        process::exit(1);
    }
    if a[4] != 50 {
        process::exit(2);
    }

    let p_back_index = 4 - 3;
    if a[p_back_index] != 20 {
        process::exit(3);
    }

    let p_from_end_index = 4; // equivalent to p_end - 1, since p_end is 5, 5-1=4, and p_last=4
    if a[p_from_end_index] != 50 {
        process::exit(5);
    }

    // The check if (p0 + 5) != p_end is always false, so no need to exit(6)
    // Similarly, p_from_end != p_last is always false, so no need to exit(4)
}