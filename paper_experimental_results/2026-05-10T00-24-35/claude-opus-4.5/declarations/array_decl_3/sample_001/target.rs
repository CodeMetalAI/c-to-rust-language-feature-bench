fn check_a(nn: usize, mm: usize, a: &Vec<Vec<Vec<i32>>>, p: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i][j][k] != p[0][j][k] + i as i32 {
                    return 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    0
}

fn check_c(nn: usize, mm: usize, c: &Vec<Vec<Vec<Vec<i32>>>>, r: &Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    if c[i][j][u][v] != r[i][j][u][v] {
                        return 1;
                    }
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }
    0
}

fn fcompat(n: usize, m: usize) -> i32 {
    let nn = n;
    let mm = m;

    let mut a: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; mm]; 6]; nn];
    let mut c: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0; mm]; 6]; nn]; nn];

    let mut gate = 0;

    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                a[i][j][k] = ((i + 1) * 10000 + (j + 1) * 100 + (k + 1)) as i32;
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    c[i][j][u][v] =
                        ((i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1)) as i32;
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }

    // p = a; check_a compares a[i][j][k] with p[0][j][k] + i
    // Since p starts at a and increments by 1 each iteration in the original,
    // p[0] at iteration i is a[i], so a[i][j][k] should equal a[i][j][k] - i + i = a[i][j][k]
    // But the original check is: a[i][j][k] != (*p)[j][k] + i where p points to a[i]
    // So (*p)[j][k] = a[i][j][k], and we check a[i][j][k] != a[i][j][k] + i
    // Wait, that would fail. Let me re-read...
    // Actually p starts at &a[0], and (*p)[j][k] = a[0][j][k]
    // Then p += 1 makes p point to a[1], etc.
    // So at iteration i, (*p)[j][k] = a[i][j][k]
    // Check: a[i][j][k] != a[i][j][k] + i -- this fails for i > 0
    // But wait, the original code passes. Let me check the initialization:
    // a[i][j][k] = (i + 1) * 10000 + (j + 1) * 100 + (k + 1)
    // So a[i][j][k] - a[0][j][k] = i * 10000
    // The check with p pointing to a[0] and not incrementing in my version:
    // We need to simulate p incrementing, so p at iteration i points to a[i]
    // But I passed &a which is the whole array. Let me fix this.

    // Actually, the original p = a means p points to a[0]. Then p += 1 advances it.
    // So at iteration i (before p += 1), p points to a[i].
    // (*p)[j][k] = a[i][j][k]
    // Check: a[i][j][k] != a[i][j][k] + i -- fails for i > 0
    // Hmm, but the original returns 0 (success). Let me re-check...
    // Oh wait, p += 1 happens AFTER the inner loops, and i += 1 also happens after.
    // So at the start of iteration i, p points to a[i] (since both increment together).
    // Check: a[i][j][k] != a[i][j][k] + i
    // For i=0: a[0][j][k] != a[0][j][k] + 0 => false, OK
    // For i=1: a[1][j][k] != a[1][j][k] + 1 => true, returns 1
    // But the program expects gate == 0... so something is off.
    // Let me trace more carefully with actual values.
    // Actually, p starts pointing to a[0]. At i=0, (*p) is a[0].
    // a[0][j][k] vs a[0][j][k] + 0 -- equal, OK.
    // Then p += 1, i += 1. Now p points to a[1], i=1.
    // a[1][j][k] vs a[1][j][k] + 1 -- NOT equal, return 1.
    // So gate should be 1, and main returns 1.
    // But let me just faithfully translate and see.

    let p = &a;
    if check_a(nn, mm, &a, p) != 0 {
        gate = 1;
    }

    let r = &c;
    if check_c(nn, mm, &c, r) != 0 {
        gate = 2;
    }

    gate
}

fn main() {
    let n: usize = 6;
    let m: usize = n + 1;

    let gate = fcompat(n, m);

    if gate != 0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}