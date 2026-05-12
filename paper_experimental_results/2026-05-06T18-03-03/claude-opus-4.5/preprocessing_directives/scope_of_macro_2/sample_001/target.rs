fn max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    let i = 3;
    let j = 5;
    if max(i, j) != 5 {
        std::process::exit(1);
    }

    let x = 2.5;
    let y = 1.5;
    if max(x, y) != 2.5 {
        std::process::exit(2);
    }

    let mut k = 1;
    // In C, max(k++, 2) evaluates k++ once, so k becomes 2 and the comparison is max(1, 2) = 2
    let k_val = k;
    k += 1;
    let r = max(k_val, 2);
    if r != 2 {
        std::process::exit(3);
    }
    if k != 2 {
        std::process::exit(4);
    }

    let mut m = 3;
    let mut n = 1;
    // In C, max(m++, n++) evaluates each once: max(3, 1) = 3, then m becomes 4, n becomes 2
    let m_val = m;
    m += 1;
    let n_val = n;
    n += 1;
    let r = max(m_val, n_val);
    if r != 3 {
        std::process::exit(5);
    }
    if m != 4 {
        std::process::exit(6);
    }
    if n != 2 {
        std::process::exit(7);
    }

    std::process::exit(0);
}