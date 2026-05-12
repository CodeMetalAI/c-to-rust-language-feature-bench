struct HoldP {
    p: Vec<i32>,
}

struct HoldA {
    tag: i32,
    a: Vec<i32>,
}

fn sum_ptr(p: &[i32]) -> i32 {
    p.iter().sum()
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    a.iter().sum()
}

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
    p[1] += 5;
    p[0] + p[1] + p[2]
}

fn main() -> i32 {
    let mut y = [0; 7];
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

    let backing = [10, 20, 30];
    let x = &backing;

    if sum_ptr(x) != (10 + 20 + 30) {
        return 2;
    }

    if mutate_through_pointer(&mut backing.clone()) != (10 + 25 + 30) {
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
        let mut hp = HoldP { p: y.to_vec() };
        if hp.p[0] != 1 {
            return 6;
        }

        let ha = HoldA { tag: 0, a: y[2..].to_vec() };

        if ha.a[0] != 3 {
            return 8;
        }
    }

    if y[0] != 1 {
        return 9;
    }

    0
}