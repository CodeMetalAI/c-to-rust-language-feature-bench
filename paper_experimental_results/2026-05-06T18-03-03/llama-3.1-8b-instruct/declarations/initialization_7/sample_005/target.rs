fn sum(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i];
    }
    s
}

fn main() {
    let a = [1, 2];
    let b = [3, 4, 5];

    if a.len() != 2 {
        std::process::exit(1);
    }
    if b.len() != 3 {
        std::process::exit(2);
    }

    if sum(&a, 2) != 3 {
        std::process::exit(3);
    }
    if sum(&b, 3) != 12 {
        std::process::exit(4);
    }

    {
        let pa = &a;
        let pb = &b;
        if pa[1] != 2 {
            std::process::exit(5);
        }
        if pb[2] != 5 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}