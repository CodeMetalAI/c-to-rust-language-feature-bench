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

fn call_as_6(pf: fn(usize, usize, &[Vec<f64>]) -> f64, n: usize, m: usize, a: &[Vec<f64>]) -> f64 {
    pf(n, m, a)
}

fn f(a: &mut Vec<Vec<f64>>) {
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
        std::process::exit(1);
    }
    if r1 != 9.5 {
        std::process::exit(2);
    }

    let mut a3 = vec![
        vec![0.0; 5],
        vec![10.0, 11.0, 12.0, 13.0, 14.0],
        vec![20.0, 21.0, 22.0, 23.0, 24.0],
    ];
    f(&mut a3);

    if a3[0][0] != 1.0 {
        std::process::exit(3);
    }
    if a3[0][4] != 5.0 {
        std::process::exit(4);
    }
    if a3[1][0] != 11.0 {
        std::process::exit(5);
    }
    if a3[1][4] != 15.0 {
        std::process::exit(6);
    }
    if a3[2][0] != 21.0 {
        std::process::exit(7);
    }
    if a3[2][4] != 25.0 {
        std::process::exit(8);
    }
}