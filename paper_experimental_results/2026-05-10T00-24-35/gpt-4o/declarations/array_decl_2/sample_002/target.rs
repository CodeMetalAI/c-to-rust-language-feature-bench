struct HoldP<'a> {
    p: &'a [i32],
}

struct HoldA<'a> {
    tag: i32,
    a: &'a [i32],
}

fn sum_ptr(p: &[i32], n: usize) -> i32 {
    p.iter().take(n).sum()
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    a.iter().sum()
}

fn mutate_through_pointer(p: &mut [i32; 3]) -> i32 {
    p[1] += 5;
    p.iter().sum()
}

fn main() -> i32 {
    let mut y = [0; 7];
    let mut backing = [10, 20, 30];
    let x: &mut [i32; 3] = &mut backing;

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

    if sum_ptr(&backing, 3) != (10 + 20 + 30) {
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
        let hp = HoldP { p: &y };
        if hp.p[0] != 1 {
            return 6;
        }

        let ha = HoldA { tag: 0, a: &y };

        if (ha.a.as_ptr() as usize) != (&ha as *const _ as usize + std::mem::size_of::<i32>()) {
            return 7;
        }

        if ha.a[2] != 3 {
            return 8;
        }
    }

    if y[0] != 1 {
        return 9;
    }

    0
}