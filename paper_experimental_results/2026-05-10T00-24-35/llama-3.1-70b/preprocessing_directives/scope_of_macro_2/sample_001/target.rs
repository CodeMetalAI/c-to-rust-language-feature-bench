fn main() {
    let i: i32 = 3;
    let j: i32 = 5;
    if max(i, j)!= 5 {
        return 1;
    }

    let x: f64 = 2.5;
    let y: f64 = 1.5;
    if max(x, y)!= 2.5 {
        return 2;
    }

    let mut k: i32 = 1;
    let r: i32 = max(k, 2);
    k += 1; // Note: Rust does not allow side effects in function calls
    if r!= 2 {
        return 3;
    }
    if k!= 2 {
        return 4;
    }

    let mut m: i32 = 3;
    let mut n: i32 = 1;
    let r: i32 = max(m, n);
    m += 1;
    n += 1;
    if r!= 3 {
        return 5;
    }
    if m!= 4 {
        return 6;
    }
    if n!= 2 {
        return 7;
    }

    return 0;
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}