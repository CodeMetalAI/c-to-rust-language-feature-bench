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
    for i in 0..7 {
        y[i] = (i + 1) as i32;
    }

    if sum_arr7(&y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        exit(1);
    }

    let mut backing = [10i32, 20, 30];

    if sum_ptr(&backing, 3) != (10 + 20 + 30) {
        exit(2);
    }

    if mutate_through_pointer(&mut backing) != (10 + 25 + 30) {
        exit(3);
    }

    if backing[1] != 25 {
        exit(4);
    }

    {
        let t: &[i32] = &y;
        if t[6] != 7 {
            exit(5);
        }
    }

    {
        let hp = HoldP { p: &y };
        if hp.p[0] != 1 {
            exit(6);
        }

        let base = y.as_ptr() as usize;
        let offset = std::mem::offset_of!(HoldA, a);
        let addr_a0 = base + offset;
        let expected = base + offset;

        if addr_a0 != expected {
            exit(7);
        }

        let offset_ints = offset / std::mem::size_of::<i32>();
        let idx = offset_ints + 2;
        let a2 = y[idx];
        if a2 != 3 {
            exit(8);
        }
    }

    if y[0] != 1 {
        exit(9);
    }

    exit(0);
}