use std::process::exit;

fn main() {
    let a = [10, 20, 30, 40, 50];

    let p0: isize = 0;
    let p2: isize = p0 + 2;
    let p4: isize = p2 + 2;
    let p_last: isize = 4;
    let p_end: isize = p_last + 1;

    if a[p2 as usize] != 30 {
        exit(1);
    }
    if a[p4 as usize] != 50 {
        exit(2);
    }

    let p_back: isize = p4 - 3;
    if a[p_back as usize] != 20 {
        exit(3);
    }

    let p_from_end: isize = p_end - 1;
    if p_from_end != p_last {
        exit(4);
    }
    if a[p_from_end as usize] != 50 {
        exit(5);
    }

    if (p0 + 5) != p_end {
        exit(6);
    }

    exit(0);
}