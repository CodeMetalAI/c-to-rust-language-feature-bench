use std::process::exit;

#[repr(C)]
struct HoldA {
    tag: i32,
    a: [i32; 0],
}

struct HoldP<'a> {
    p: &'a [i32],
}

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

    let mut backing = [10i32, 20, 30];
    let x: &mut [i32] = &mut backing;

    if sum_ptr(&x, 3) != (10 + 20 + 30) {
        exit(2);
    }

    if mutate_through_pointer(x) != (10 + 25 + 30) {
        exit(3);
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
        let hp = HoldP { p: &y };
        if hp.p[0] != 1 {
            exit(6);
        }

        let ha_base = y.as_ptr() as usize;
        let offset = std::mem::offset_of!(HoldA, a);
        let ha_a0_addr = ha_base + offset;
        let ha_plus_offset = ha_base + offset;

        if ha_a0_addr != ha_plus_offset {
            exit(7);
        }

        if y[2] != 3 {
            exit(8);
        }
    }

    if y[0] != 1 {
        exit(9);
    }

    exit(0);
}