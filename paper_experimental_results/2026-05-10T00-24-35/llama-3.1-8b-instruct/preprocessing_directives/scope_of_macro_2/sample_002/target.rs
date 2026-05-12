macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a
        } else {
            $b
        }
    };
}

fn main() -> i32 {
    let i = 3;
    let j = 5;
    if max!(i, j) != 5 {
        return 1;
    }

    let x = 2.5;
    let y = 1.5;
    if max!(x, y) != 2.5 {
        return 2;
    }

    let mut k = 1;
    let r = max!(k, 2);
    if r != 2 {
        return 3;
    }
    if k != 2 {
        return 4;
    }

    let m = 3;
    let n = 1;
    let mut r = max!(m, n);
    if r != 3 {
        return 5;
    }
    if m != 4 {
        return 6;
    }
    if n != 2 {
        return 7;
    }

    0
}