fn sum3(a: &[i32]) -> i32 {
    a[0] + a[1] + a[2]
}

fn main() {
    let x = [1, 3, 5];

    if x.len() != 3 {
        std::process::exit(1);
    }

    if x[0] != 1 || x[1] != 3 || x[2] != 5 {
        std::process::exit(2);
    }

    if sum3(&x) != 9 {
        std::process::exit(3);
    }

    let mut p = &x;
    if *p.last().unwrap() != 5 {
        std::process::exit(4);
    }

    std::process::exit(0);
}