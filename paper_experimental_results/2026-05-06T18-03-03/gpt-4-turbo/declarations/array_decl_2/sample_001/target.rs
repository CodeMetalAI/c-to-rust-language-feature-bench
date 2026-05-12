fn main() {
    let mut y = [0; 7];
    let mut backing = [10, 20, 30];
    let x = &mut backing;

    y[0] = 1;
    y[1] = 2;
    y[2] = 3;
    y[3] = 4;
    y[4] = 5;
    y[5] = 6;
    y[6] = 7;

    if sum_arr7(&y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        std::process::exit(1);
    }

    if sum_ptr(x, 3) != (10 + 20 + 30) {
        std::process::exit(2);
    }

    if mutate_through_pointer(x) != (10 + 25 + 30) {
        std::process::exit(3);
    }

    if backing[1] != 25 {
        std::process::exit(4);
    }

    {
        let t = &y;
        if t[6] != 7 {
            std::process::exit(5);
        }
    }

    {
        let hp = HoldP { p: &y };
        if hp.p[0] != 1 {
            std::process::exit(6);
        }

        let ha = unsafe { &*(y.as_ptr() as *const HoldA) };
        let ha_a_offset = offset_of!(HoldA, a);

        if &ha.a as *const _ as usize != ha as *const _ as usize + ha_a_offset {
            std::process::exit(7);
        }

        if ha.a[2] != 3 {
            std::process::exit(8);
        }
    }

    if y[0] != 1 {
        std::process::exit(9);
    }
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

struct HoldP<'a> {
    p: &'a [i32],
}

#[repr(C)]
struct HoldA {
    tag: i32,
    a: [i32],
}

#[macro_use]
extern crate memoffset;