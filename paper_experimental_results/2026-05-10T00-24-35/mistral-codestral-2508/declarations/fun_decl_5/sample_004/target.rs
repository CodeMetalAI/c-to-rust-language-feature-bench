fn maximum(n: usize, m: usize, a: &[Vec<f64>]) -> f64 {
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

fn call_as_6<F>(pf: F, n: usize, m: usize, a: &[Vec<f64>]) -> f64
where
    F: Fn(usize, usize, &[Vec<f64>]) -> f64,
{
    pf(n, m, a)
}

fn f(a: &mut [Vec<f64>]) {
    for i in 0..3 {
        a[i][0] += 1.0;
        a[i][4] += 1.0;
    }
}

fn main() -> i32 {
    let n = 4;
    let m = 6;

    let mut mat: Vec<Vec<f64>> = vec![vec![0.0; m]; n];
    mat[2][3] = 9.5;
    mat[1][5] = 7.0;

    let r0 = maximum(n, m, &mat);
    let r1 = call_as_6(maximum, n, m, &mat);

    if r0 != 9.5 {
        return 1;
    }
    if r1 != 9.5 {
        return 2;
    }

    let mut a3: Vec<Vec<f64>> = vec![vec![0.0; 5]; 3];
    for i in 0..3 {
        for j in 0..5 {
            a3[i][j] = (i * 10 + j) as f64;
        }
    }

    f(&mut a3);

    if a3[0][0] != 1.0 {
        return 3;
    }
    if a3[0][4] != 5.0 {
        return 4;
    }
    if a3[1][0] != 11.0 {
        return 5;
    }
    if a3[1][4] != 15.0 {
        return 6;
    }
    if a3[2][0] != 21.0 {
        return 7;
    }
    if a3[2][4] != 25.0 {
        return 8;
    }

    0
}