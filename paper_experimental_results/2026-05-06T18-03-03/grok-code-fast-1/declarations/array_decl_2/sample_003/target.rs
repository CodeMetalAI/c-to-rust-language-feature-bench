fn sum_ptr(p: &[i32], n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += p[i as usize];
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

fn main() -> i32 {
    let mut y: [i32; 7] = [0; 7];
    let mut backing: [i32; 3] = [10, 20, 30];
    let x = &mut backing;
    y[0] = 1;
    y[1] = 2;
    y[2] = 3;
    y[3] = 4;
    y[4] = 5;
    y[5] = 6;
    y[6] = 7;
    if sum_arr7(&y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        return 1;
    }
    if sum_ptr(x, 3) != (10 + 20 + 30) {
        return 2;
    }
    if mutate_through_pointer(x) != (10 + 25 + 30) {
        return 3;
    }
    if backing[1] != 25 {
        return 4;
    }
    {
        let t = &y;
        if t[6] != 7 {
            return 5;
        }
    }
    {
        struct HoldP<'a> {
            p: &'a mut [i32],
        }
        struct HoldA<'a> {
            tag: i32,
            a: &'a [i32],
        }
        let mut hp = HoldP { p: &mut y };
        if hp.p[0] != 1 {
            return 6;
        }
        let ha = HoldA {
            tag: y[0],
            a: &y[1..],
        };
        if ha.a[2] != 3 {
            return 8;
        }
    }
    if y[0] != 1 {
        return 9;
    }
    0
}