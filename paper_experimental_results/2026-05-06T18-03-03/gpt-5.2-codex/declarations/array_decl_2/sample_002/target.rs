use std::mem::{align_of, size_of};
use std::process::exit;

fn sum_ptr(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += p[i];
        i += 1;
    }
    s
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < 7 {
        s += a[i];
        i += 1;
    }
    s
}

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
    p[1] += 5;
    p[0] + p[1] + p[2]
}

struct HoldP<'a> {
    p: &'a [i32],
}

fn main() {
    let mut y = [0i32; 7];
    y[0] = 1;
    y[1] = 2;
    y[2] = 3;
    y[3] = 4;
    y[4] = 5;
    y[5] = 6;
    y[6] = 7;

    if sum_arr7(&y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        exit(1);
    }

    let mut backing = [10i32, 20i32, 30i32];

    {
        let x = &backing;
        if sum_ptr(x, 3) != (10 + 20 + 30) {
            exit(2);
        }
    }

    {
        let x = &mut backing;
        if mutate_through_pointer(x) != (10 + 25 + 30) {
            exit(3);
        }
    }

    if backing[1] != 25 {
        exit(4);
    }

    {
        let t = &y;
        if t[6] != 7 {
            exit(5);
        }
    }

    {
        let mut hp = HoldP { p: &[] };
        hp.p = &y;
        if hp.p[0] != 1 {
            exit(6);
        }

        // Simulate HoldA layout: tag at index 0, flexible array starts at index 1
        let a0_index = 1usize;

        let align = align_of::<i32>();
        let mut offset = size_of::<i32>();
        if offset % align != 0 {
            offset += align - (offset % align);
        }

        if a0_index * size_of::<i32>() != offset {
            exit(7);
        }

        let a2 = y[a0_index + 2];
        if a2 != 3 {
            exit(8);
        }
    }

    if y[0] != 1 {
        exit(9);
    }

    exit(0);
}