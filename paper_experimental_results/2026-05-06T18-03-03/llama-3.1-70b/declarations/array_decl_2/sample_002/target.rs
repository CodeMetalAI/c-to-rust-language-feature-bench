fn main() {
    let mut y = [0; 7];
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

    let x = [10, 20, 30];
    if sum_ptr(&x, 3) != (10 + 20 + 30) {
        std::process::exit(2);
    }

    let mut x = [10, 20, 30];
    if mutate_through_pointer(&mut x) != (10 + 25 + 30) {
        std::process::exit(3);
    }

    if x[1] != 25 {
        std::process::exit(4);
    }

    {
        let t = &y;
        if t[6] != 7 {
            std::process::exit(5);
        }
    }

    {
        struct HoldP {
            p: *const i32,
        }

        struct HoldA {
            tag: i32,
            a: [i32; 0],
        }

        let hp = HoldP { p: y.as_ptr() };
        if unsafe { *hp.p.offset(0) } != 1 {
            std::process::exit(6);
        }

        let ha = y.as_ptr() as *const HoldA;
        if (ha as *const u8).offset(1) != ha.offset(1) as *const u8 {
            std::process::exit(7);
        }

        if unsafe { *ha.offset(1).offset(2) } != 3 {
            std::process::exit(8);
        }
    }

    if y[0] != 1 {
        std::process::exit(9);
    }

    std::process::exit(0);
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    let mut s = 0;
    for &x in a {
        s += x;
    }
    s
}

fn sum_ptr(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i];
    }
    s
}

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
    p[1] += 5;
    p[0] + p[1] + p[2]
}