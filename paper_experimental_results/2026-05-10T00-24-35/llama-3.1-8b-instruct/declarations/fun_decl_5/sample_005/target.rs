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

fn f(a: &[[f64; 5]; 3]) {
    for i in 0..3 {
        a[i][0] += 1.0;
        a[i][4] += 1.0;
    }
}

fn main() {
    let n = 4;
    let m = 6;
    let mut mat: [[f64; m]; n] = [[0.0; m]; n];
    mat[2][3] = 9.5;
    mat[1][5] = 7.0;

    let r0 = maximum(n, m, &mat);
    let r1: f64 = unsafe { maximum(n, m, &mat as *const _ as *const [[f64; m]; n]) };

    if r0 != 9.5 {
        std::process::exit(1);
    }
    if r1 != 9.5 {
        std::process::exit(2);
    }

    let a3: [[f64; 5]; 3] = [
        [0.0, 10.0, 20.0, 30.0, 40.0],
        [50.0, 60.0, 70.0, 80.0, 90.0],
        [100.0, 110.0, 120.0, 130.0, 140.0],
    ];

    f(&a3);

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