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
    let r = max(k, 2);
    if r != 2 {
        std::process::exit(3);
    }
    if k != 1 {
        std::process::exit(4);
    }

    let mut m = 3;
    let mut n = 1;
    let r = max(m, n);
    if r != 3 {
        std::process::exit(5);
    }
    if m != 3 {
        std::process::exit(6);
    }
    if n != 1 {
        std::process::exit(7);
    }

    std::process::exit(0);
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}