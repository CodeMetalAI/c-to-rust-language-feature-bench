use std::process::exit;

const MAX: usize = 12;

fn main() {
    let a: [i32; MAX] = [1, 3, 5, 7, 9, 0, 0, 8, 6, 4, 2, 0];

    if a[0] != 1 {
        exit(1);
    }
    if a[1] != 3 {
        exit(2);
    }
    if a[2] != 5 {
        exit(3);
    }
    if a[3] != 7 {
        exit(4);
    }
    if a[4] != 9 {
        exit(5);
    }

    if a[MAX - 5] != 8 {
        exit(6);
    }
    if a[MAX - 4] != 6 {
        exit(7);
    }
    if a[MAX - 3] != 4 {
        exit(8);
    }
    if a[MAX - 2] != 2 {
        exit(9);
    }
    if a[MAX - 1] != 0 {
        exit(10);
    }

    let mut i = 5usize;
    while i < MAX - 5 {
        if a[i] != 0 {
            exit(11);
        }
        i += 1;
    }

    exit(0);
}