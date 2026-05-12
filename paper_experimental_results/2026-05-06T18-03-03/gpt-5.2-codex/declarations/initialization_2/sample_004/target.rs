use std::process::exit;

fn sum3(a: &[i32; 3]) -> i32 {
    a[0] + a[1] + a[2]
}

fn main() {
    let x = [1, 3, 5];

    if x.len() != 3 {
        exit(1);
    }

    if x[0] != 1 || x[1] != 3 || x[2] != 5 {
        exit(2);
    }

    if sum3(&x) != 9 {
        exit(3);
    }

    {
        let p: &[i32] = &x;
        if p[2] != 5 {
            exit(4);
        }
    }

    exit(0);
}