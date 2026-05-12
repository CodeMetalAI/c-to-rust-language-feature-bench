fn main() {
    let mut y = [0; 7];
    y[0] = 1;
    y[1] = 2;
    y[2] = 3;
    y[3] = 4;
    y[4] = 5;
    y[5] = 6;
    y[6] = 7;

    if sum_arr7(&y)!= (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        return;
    }

    let x = [10, 20, 30];
    if sum_ptr(&x, 3)!= (10 + 20 + 30) {
        return;
    }

    let mut backing = [10, 20, 30];
    if mutate_through_pointer(&mut backing)!= (10 + 25 + 30) {
        return;
    }

    if backing[1]!= 25 {
        return;
    }

    {
        let t = &y;
        if t[6]!= 7 {
            return;
        }
    }

    {
        struct HoldP {
            p: *const i32,
        }

        struct HoldA {
            tag: i32,
            a: [i32],
        }

        let hp = HoldP { p: y.as_ptr() };
        if unsafe { *hp.p }!= 1 {
            return;
        }

        let ha = y.as_ptr() as *const HoldA;
        if (ha as *const u8).offset(1)!= (ha as *const u8) {
            return;
        }

        if unsafe { (*ha).a[2] }!= 3 {
            return;
        }
    }

    if y[0]!= 1 {
        return;
    }
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    let mut s = 0;
    for i in 0..7 {
        s += a[i];
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