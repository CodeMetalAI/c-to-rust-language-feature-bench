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
    let mut y: [i32; 7] = [0; 7];
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

    let mut backing = [10, 20, 30];
    let x = &mut backing;

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
        let mut hp = HoldP { p: &mut y };
        if hp.p[0] != 1 {
            std::process::exit(6);
        }

        let ha = unsafe { &*(y.as_ptr() as *const HoldA) };

        if (ha.a.as_ptr() as *const u8 as usize) != (ha as *const HoldA as *const u8 as usize + std::mem::size_of::<i32>()) {
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

struct HoldP<'a> {
    p: &'a mut [i32],
}

struct HoldA {
    tag: i32,
    a: [i32; 0],
}