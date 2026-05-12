fn main() {
    macro_rules! max {
        ($a:expr, $b:expr) => {
            if $a > $b { $a } else { $b }
        };
    }

    let mut i: i32 = 3;
    let mut j: i32 = 5;
    if max!(i, j) != 5 {
        std::process::exit(1);
    }

    let mut x: f64 = 2.5;
    let mut y: f64 = 1.5;
    if max!(x, y) != 2.5 {
        std::process::exit(2);
    }

    let mut k: i32 = 1;
    let r = max!({ let old = k; k += 1; old }, 2);
    if r != 2 {
        std::process::exit(3);
    }
    if k != 2 {
        std::process::exit(4);
    }

    let mut m: i32 = 3;
    let mut n: i32 = 1;
    let r = max!({ let old = m; m += 1; old }, { let old = n; n += 1; old });
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