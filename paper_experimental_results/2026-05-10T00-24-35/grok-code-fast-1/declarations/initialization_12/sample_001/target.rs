use std::process;

const MAX: usize = 12;

fn main() {
    let a: [i32; MAX] = [1, 3, 5, 7, 9, 0, 0, 8, 6, 4, 2, 0];

    if a[0] != 1 {
        process::exit(1);
    }
    if a[1] != 3 {
        process::exit(2);
    }
    if a[2] != 5 {
        process::exit(3);
    }
    if a[3] != 7 {
        process::exit(4);
    }
    if a[4] != 9 {
        process::exit(5);
    }

    if a[MAX - 5] != 8 {
        process::exit(6);
    }
    if a[MAX - 4] != 6 {
        process::exit(7);
    }
    if a[MAX - 3] != 4 {
        process::exit(8);
    }
    if a[MAX - 2] != 2 {
        process::exit(9);
    }
    if a[MAX - 1] != 0 {
        process::exit(10);
    }

    {
        let mut i = 5;
        while i < MAX - 5 {
            if a[i] != 0 {
                process::exit(11);
            }
            i += 1;
        }
    }

    process::exit(0);
}