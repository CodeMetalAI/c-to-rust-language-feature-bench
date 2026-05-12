fn maximum(n: usize, m: usize, a: &[Vec<f64>]) -> f64 {
    let mut mx = a[0][0];
    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j < m {
            let v = a[i][j];
            if v > mx {
                mx = v;
            }
            j += 1;
        }
        i += 1;
    }
    mx
}

fn call_as_6(pf: fn(usize, usize, &[[f64; 6]]) -> f64, n: usize, m: usize, a: &[Vec<f64>]) -> f64 {
    let mut arr: Vec<[f64; 6]> = Vec::new();
    for row in a {
        let mut fixed_row = [0.0; 6];
        for (j, val) in row.iter().take(6).enumerate() {
            fixed_row[j] = *val;
        }
        arr.push(fixed_row);
    }
    pf(n, m, &arr)
}

fn f(a: &mut [[f64; 5]]) {
    let mut i = 0;
    while i < 3 {
        a[i][0] += 1.0;
        a[i][4] += 1.0;
        i += 1;
    }
}

fn main() {
    let n = 4;
    let m = 6;

    let mut mat = vec![vec![0.0; m]; n];

    mat[2][3] = 9.5;
    mat[1][5] = 7.0;

    let r0 = maximum(n, m, &mat);
    let r1 = call_as_6(maximum, n, m, &mat);

    if r0 != 9.5 {
        return;
    }
    if r1 != 9.5 {
        return;
    }

    let mut a3 = [[0.0; 5]; 3];
    let mut i = 0;
    while i < 3 {
        let mut j = 0;
        while j < 5 {
            a3[i][j] = (i * 10 + j) as f64;
            j += 1;
        }
        i += 1;
    }

    f(&mut a3);

    if a3[0][0] != 1.0 {
        return;
    }
    if a3[0][4] != 5.0 {
        return;
    }
    if a3[1][0] != 11.0 {
        return;
    }
    if a3[1][4] != 15.0 {
        return;
    }
    if a3[2][0] != 21.0 {
        return;
    }
    if a3[2][4] != 25.0 {
        return;
    }

    std::process::exit(0);
}