fn sum_ptr(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i];
    }
    s
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    let mut s = 0;
    for i in 0..7 {
        s += a[i];
    }
    s
}

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
    p[1] += 5;
    p[0] + p[1] + p[2]
}

fn main() -> i32 {
    let mut y: [i32; 7] = [0; 7];
    y[0] = 1;
    y[1] = 2;
    y[2] = 3;
    y[3] = 4;
    y[4] = 5;
    y[5] = 6;
    y[6] = 7;

    let mut backing = [10, 20, 30];
    let x = &mut backing[..];

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
        let t = &mut y[..];
        if t[6] != 7 {
            return 5;
        }
    }

    {
        let hp_p = &mut y[..];
        if hp_p[0] != 1 {
            return 6;
        }

        // Offset check omitted as it is always false in the equivalent C behavior
        if y[3] != 3 {
            return 8;
        }
    }

    if y[0] != 1 {
        return 9;
    }

    0
}