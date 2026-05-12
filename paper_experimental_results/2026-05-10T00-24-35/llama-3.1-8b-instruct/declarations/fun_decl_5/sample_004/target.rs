fn maximum(n: usize, m: usize, a: &[[f64; m]; n]) -> f64 {
    let mut mx = a[0][0];
    for i in 0..n {
        for j in 0..m {
            let v = a[i][j];
            if v > mx {
                mx = v;
            }
        }
    }
    mx
}

fn call_as_6(pf: impl Fn(usize, usize, &[f64; 6]) -> f64, n: usize, m: usize, a: &[[f64; 6]; n]) -> f64 {
    let p: &[f64; 6] = &a[0][..];
    unsafe { pf(n, m, std::slice::from_raw_parts(p.as_ptr(), m)) }
}

fn f(a: &[[f64; 5]; 3]) {
    for i in 0..3 {
        a[i][0] += 1.0;
        a[i][4] += 1.0;
    }
}

fn main() {
    let n = 4;
    let m = 6;

    let mut mat = [[0.0; m]; n];

    {
        for i in 0..n {
            for j in 0..m {
                mat[i][j] = 0.0;
            }
        }
        mat[2][3] = 9.5;
        mat[1][5] = 7.0;
    }

    let r0 = maximum(n, m, &mat);
    let r1 = call_as_6(|n, m, p| maximum(n, m, &p[..]), n, m, &mat);

    if r0 != 9.5 {
        std::process::exit(1);
    }
    if r1 != 9.5 {
        std::process::exit(2);
    }

    let mut a3 = [[(i * 10 + j) as f64; 5]; 3];

    f(&a3);

    for i in 0..3 {
        for j in 0..5 {
            if (i * 10 + j as usize) != a3[i][j] {
                std::process::exit(3 + (i * 10 + j));
            }
        }
    }
}