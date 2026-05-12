const N: usize = 6;
const M: usize = 7;

type AType = [[[i32; M]; 6]; N];
type CType = [[[[i32; M]; 6]; N]; N];

fn check_a(nn: usize, mm: usize, a: &AType, p_idx: usize) -> i32 {
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                if a[i][j][k] != a[p_idx + i][j][k] + i as i32 {
                    return 1;
                }
            }
        }
    }
    0
}

fn check_c(nn: usize, mm: usize, c: &CType, r: &CType) -> i32 {
    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    if c[i][j][u][v] != r[i][j][u][v] {
                        return 1;
                    }
                }
            }
        }
    }
    0
}

fn fcompat(gate: &mut i32) {
    let nn = N;
    let mm = M;

    let mut a: AType = [[[0; M]; 6]; N];

    let mut c: CType = [[[[0; M]; 6]; N]; N];

    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                a[i][j][k] = (i as i32 + 1) * 10000 + (j as i32 + 1) * 100 + (k as i32 + 1);
            }
        }
    }

    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    c[i][j][u][v] = (i as i32 + 1) * 1000000 + (j as i32 + 1) * 10000 + (u as i32 + 1) * 100 + (v as i32 + 1);
                }
            }
        }
    }

    let p_idx = 0;
    if check_a(nn, mm, &a, p_idx) != 0 {
        *gate = 1;
    }

    if check_c(nn, mm, &c, &c) != 0 {
        *gate = 2;
    }
}

fn main() {
    let mut gate = 0;
    fcompat(&mut gate);

    if gate != 0 {
        std::process::exit(1);
    }
}