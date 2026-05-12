fn sum(p: &[i32]) -> i32 {
    p.iter().sum()
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

    if sum(&a) != 3 {
        std::process::exit(3);
    }
    if sum(&b) != 12 {
        std::process::exit(4);
    }

    {
        let pa = &a[0];
        let pb = &b[0];
        if unsafe { *pa.offset(1) } != 2 {
            std::process::exit(5);
        }
        if unsafe { *pb.offset(2) } != 5 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}