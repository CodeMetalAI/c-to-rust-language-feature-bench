fn main() {
    let (i, j) = (3, 5);
    if i.max(j) != 5 {
        std::process::exit(1);
    }

    let (x, y) = (2.5, 1.5);
    if x.max(y) != 2.5 {
        std::process::exit(2);
    }

    let mut k = 1;
    let r = (k + 1).max(2);
    k += 1;
    if r != 2 {
        std::process::exit(3);
    }
    if k != 2 {
        std::process::exit(4);
    }

    let (mut m, mut n) = (3, 1);
    let r = (m + 1).max(n + 1);
    m += 1;
    n += 1;
    if r != 3 {
        std::process::exit(5);
    }
    if m != 4 {
        std::process::exit(6);
    }
    if n != 2 {
        std::process::exit(7);
    }

    // Successful completion
}