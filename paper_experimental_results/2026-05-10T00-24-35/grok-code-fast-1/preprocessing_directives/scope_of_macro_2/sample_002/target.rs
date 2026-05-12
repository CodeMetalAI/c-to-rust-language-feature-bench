fn main() {
    let i = 3;
    let j = 5;
    if std::cmp::max(i, j) != 5 {
        std::process::exit(1);
    }

    let x: f64 = 2.5;
    let y: f64 = 1.5;
    if x.max(y) != 2.5 {
        std::process::exit(2);
    }

    let mut k = 1;
    let r = std::cmp::max({ let temp = k; k += 1; temp }, 2);
    if r != 2 {
        std::process::exit(3);
    }
    if k != 2 {
        std::process::exit(4);
    }

    let mut m = 3;
    let mut n = 1;
    let r = std::cmp::max({ let temp = m; m += 1; temp }, { let temp = n; n += 1; temp });
    if r != 3 {
        std::process::exit(5);
    }
    if m != 4 {
        std::process::exit(6);
    }
    if n != 2 {
        std::process::exit(7);
    }
}