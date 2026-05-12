fn sum3(a: &[i32; 3]) -> i32 {
    a[0] + a[1] + a[2]
}

fn main() {
    let x: [i32; 3] = [1, 3, 5];

    // In Rust, [i32; 3] is always exactly 3 elements, so no need for sizeof check.

    if x[0] != 1 || x[1] != 3 || x[2] != 5 {
        std::process::exit(2);
    }

    if sum3(&x) != 9 {
        std::process::exit(3);
    }

    {
        let p: &[i32; 3] = &x;
        if p[2] != 5 {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}