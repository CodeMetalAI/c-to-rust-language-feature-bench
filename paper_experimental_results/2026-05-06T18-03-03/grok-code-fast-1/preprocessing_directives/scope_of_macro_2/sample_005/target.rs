macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
}

fn main() {
    let mut i = 3;
    let mut j = 5;
    if max!(i, j) != 5 {
        std::process::exit(1);
    }

    let mut x: f64 = 2.5;
    let mut y: f64 = 1.5;
    if max!(x, y) != 2.5 {
        std::process::exit(2);
    }

    let mut k = 1;
    let r = max!({ let temp = k; k += 1; temp }, 2);
    if r != 2 {
        std::process::exit(3);
    }
    if k != 2 {
        std::process::exit(4);
    }

    let mut m = 3;
    let mut n = 1;
    let r = max!({ let temp = m; m += 1; temp }, { let temp = n; n += 1; temp });
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