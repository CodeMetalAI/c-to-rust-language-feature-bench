fn sum(p: &[i32]) -> i32 {
    p.iter().sum()
}

fn main() {
    let a: [i32; 2] = [1, 2];
    let b: [i32; 3] = [3, 4, 5];

    if a.len()!= 2 {
        std::process::exit(1);
    }
    if b.len()!= 3 {
        std::process::exit(2);
    }

    if sum(&a)!= 3 {
        std::process::exit(3);
    }
    if sum(&b)!= 12 {
        std::process::exit(4);
    }

    let pa: &[i32] = &a;
    let pb: &[i32] = &b;
    if pa[1]!= 2 {
        std::process::exit(5);
    }
    if pb[2]!= 5 {
        std::process::exit(6);
    }

    std::process::exit(0);
}