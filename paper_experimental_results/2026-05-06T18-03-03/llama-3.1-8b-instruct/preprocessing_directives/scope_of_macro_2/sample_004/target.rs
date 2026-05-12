fn main() {
    let i = 3;
    let j = 5;
    if i.max(j) != 5 {
        return 1;
    }

    let x = 2.5;
    let y = 1.5;
    if x.max(y) != 2.5 {
        return 2;
    }

    let mut k = 1;
    let r = k.max(2);
    if r != 2 {
        return 3;
    }
    if k != 2 {
        return 4;
    }

    let mut m = 3;
    let mut n = 1;
    let r = m.max(n);
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

// Define a macro to calculate the maximum of two values
macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a
        } else {
            $b
        }
    };
}