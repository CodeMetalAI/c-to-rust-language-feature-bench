fn sum3(a: &[i32; 3]) -> i32 {
    a[0] + a[1] + a[2]
}

fn main() {
    let x = [1, 3, 5];

    if x.len() != 3 {
        return 1;
    }

    if x[0] != 1 || x[1] != 3 || x[2] != 5 {
        return 2;
    }

    if sum3(&x) != 9 {
        return 3;
    }

    {
        let p = &x[..];
        if p[2] != 5 {
            return 4;
        }
    }

    0
}