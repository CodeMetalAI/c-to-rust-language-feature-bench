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
}