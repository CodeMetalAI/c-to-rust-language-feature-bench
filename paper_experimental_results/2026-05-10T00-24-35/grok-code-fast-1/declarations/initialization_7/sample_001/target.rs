fn sum(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += p[i];
        i += 1;
    }
    s
}

fn main() -> i32 {
    let a: [i32; 2] = [1, 2];
    let b: [i32; 3] = [3, 4, 5];

    if a.len() != 2 {
        return 1;
    }
    if b.len() != 3 {
        return 2;
    }

    if sum(&a, 2) != 3 {
        return 3;
    }
    if sum(&b, 3) != 12 {
        return 4;
    }

    {
        let pa: &[i32] = &a;
        let pb: &[i32] = &b;
        if pa[1] != 2 {
            return 5;
        }
        if pb[2] != 5 {
            return 6;
        }
    }

    0
}