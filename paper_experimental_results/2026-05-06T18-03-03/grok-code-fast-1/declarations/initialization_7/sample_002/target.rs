fn sum(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += p[i];
        i += 1;
    }
    s
}

fn main() {
    let a: [i32; 2] = [1, 2];
    let b: [i32; 3] = [3, 4, 5];

    if std::mem::size_of_val(&a) / std::mem::size_of::<i32>() != 2 {
        std::process::exit(1);
    }
    if std::mem::size_of_val(&b) / std::mem::size_of::<i32>() != 3 {
        std::process::exit(2);
    }

    if sum(&a, 2) != 3 {
        std::process::exit(3);
    }
    if sum(&b, 3) != 12 {
        std::process::exit(4);
    }

    {
        let pa: &[i32] = &a;
        let pb: &[i32] = &b;
        if pa[1] != 2 {
            std::process::exit(5);
        }
        if pb[2] != 5 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}