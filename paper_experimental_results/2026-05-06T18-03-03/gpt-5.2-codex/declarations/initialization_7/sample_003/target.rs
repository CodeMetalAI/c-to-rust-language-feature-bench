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

    let mut code = 0;

    if a.len() != 2 {
        code = 1;
    } else if b.len() != 3 {
        code = 2;
    } else if sum(&a, 2) != 3 {
        code = 3;
    } else if sum(&b, 3) != 12 {
        code = 4;
    } else {
        let pa = &a;
        let pb = &b;
        if pa[1] != 2 {
            code = 5;
        } else if pb[2] != 5 {
            code = 6;
        }
    }

    std::process::exit(code);
}