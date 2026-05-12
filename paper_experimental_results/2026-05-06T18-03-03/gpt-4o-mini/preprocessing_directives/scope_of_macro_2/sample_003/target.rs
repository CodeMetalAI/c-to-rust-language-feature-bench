fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() -> i32 {
    let mut i = 3;
    let j = 5;
    if max(i, j) != 5 {
        return 1;
    }

    let x = 2.5;
    let y = 1.5;
    if max(x, y) != 2.5 {
        return 2;
    }

    let mut k = 1;
    let r = max(k, 2);
    k += 1; // Increment k after using it
    if r != 2 {
        return 3;
    }
    if k != 2 {
        return 4;
    }

    let mut m = 3;
    let mut n = 1;
    let r = max(m, n);
    m += 1; // Increment m after using it
    n += 1; // Increment n after using it
    if r != 3 {
        return 5;
    }
    if m != 4 {
        return 6;
    }
    if n != 2 {
        return 7;
    }

    return 0;
}