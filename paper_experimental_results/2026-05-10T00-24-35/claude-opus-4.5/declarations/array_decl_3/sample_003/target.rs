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
    // Since p starts at a and increments by 1 each iteration in check_a,
    // p[0] at iteration i is a[i], so a[i][j][k] should equal a[i][j][k] - effectively a[0][j][k] + i
    // But the original code does: a[i][j][k] != (*p)[j][k] + i where p starts at &a[0] and p += 1 each i
    // So at iteration i, p points to a[i], so (*p)[j][k] = a[i][j][k], and we check a[i][j][k] != a[i][j][k] + i
    // Wait, that would fail. Let me re-read...
    // Actually p starts at a (which is &a[0]), and in the loop p += 1 happens AFTER the inner checks
    // So at i=0, p points to a[0], checking a[0][j][k] != a[0][j][k] + 0 = a[0][j][k], OK
    // At i=1, p points to a[1], checking a[1][j][k] != a[1][j][k] + 1, which would fail
    // Unless... the + i is meant differently. Let me trace: a[i][j][k] = (i+1)*10000 + ...
    // And we compare with (*p)[j][k] + i where after incrementing p, (*p) = a[i]
    // Hmm, p increments after the check, so at i=0, p=a[0]; at i=1, p=a[1], etc.
    // So a[i][j][k] vs a[i][j][k] + i - this fails for i > 0
    // Re-reading: p += 1 is after i += 1, so during iteration i, p still points to a[i]
    // Actually no - p is incremented at the END of each iteration, so:
    // i=0: p=&a[0], check a[0] vs a[0]+0, then p=&a[1], i=1
    // i=1: p=&a[1], check a[1] vs a[1]+1, FAIL
    // This seems like a bug in original code or I'm misreading. Let me just translate literally.

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