fn sum(p: &[i32]) -> i32 {
    let mut s = 0;
    for &value in p {
        s += value;
    }
    s
}

fn main() -> i32 {
    let a = [1, 2];
    let b = [3, 4, 5];

    if a.len() != 2 {
        return 1;
    }
    if b.len() != 3 {
        return 2;
    }

    if sum(&a) != 3 {
        return 3;
    }
    if sum(&b) != 12 {
        return 4;
    }

    {
        let pa = &a;
        let pb = &b;
        if pa[1] != 2 {
            return 5;
        }
        if pb[2] != 5 {
            return 6;
        }
    }

    0
}