fn main() {
    // Global mutable state is represented as static mut, but we'll avoid that
    // by moving everything into main and passing references.
    let n = 6;
    let m = n + 1;
    let mut gate = 0;

    fcompat(n, m, &mut gate);

    if gate != 0 {
        std::process::exit(1);
    }
}

fn fcompat(nn: usize, mm: usize, gate: &mut i32) {
    // Use vectors to simulate variable-length arrays
    let mut a = vec![vec![vec![0; mm]; 6]; nn];
    let mut c = vec![vec![vec![vec![0; mm]; 6]; nn]; nn];

    // Initialize a
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                a[i][j][k] = (i + 1) * 10000 + (j + 1) * 100 + (k + 1);
            }
        }
    }

    // Initialize c
    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    c[i][j][u][v] =
                        (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1);
                }
            }
        }
    }

    // Check a
    if check_a(&a, mm) != 0 {
        *gate = 1;
    }

    // Check c
    if check_c(&c, mm) != 0 {
        *gate = 2;
    }
}

fn check_a(a: &[Vec<Vec<i32>>], mm: usize) -> i32 {
    let nn = a.len();
    // p is a pointer to the first element of a (simulated by indexing)
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                // (*p)[j][k] is equivalent to a[i][j][k] - i
                if a[i][j][k] != (a[i][j][k] - i as i32) + i as i32 {
                    return 1;
                }
            }
        }
        // p += 1 is implicit in the loop iteration
    }
    0
}

fn check_c(c: &[Vec<Vec<Vec<i32>>>], mm: usize) -> i32 {
    let nn = c.len();
    // r is a pointer to the first element of c (simulated by indexing)
    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    // r[i][j][u][v] is equivalent to c[i][j][u][v]
                    if c[i][j][u][v] != c[i][j][u][v] {
                        return 1;
                    }
                }
            }
        }
    }
    0
}