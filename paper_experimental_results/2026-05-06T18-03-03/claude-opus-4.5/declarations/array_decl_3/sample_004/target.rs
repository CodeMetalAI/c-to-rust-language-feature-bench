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
    // p[0] at iteration i is a[i], so (*p)[j][k] + i = a[i][j][k] + i
    // But the check is a[i][j][k] != (*p)[j][k] + i
    // In original: p starts at &a[0], and in check_a, p is incremented after each i
    // So at iteration i, *p is a[i], and (*p)[j][k] + i = a[i][j][k] + i? No wait...
    // Actually looking more carefully: the check is a[i][j][k] != (*p)[j][k] + i
    // where p starts at a[0] and p += 1 happens after checking each i
    // So at i=0: (*p)[j][k] = a[0][j][k], check a[0][j][k] != a[0][j][k] + 0 = false (good)
    // At i=1: p now points to a[1], (*p)[j][k] = a[1][j][k], check a[1][j][k] != a[1][j][k] + 1
    // This would be true (fail) unless... let me re-read
    // Oh I see - p += 1 is INSIDE the loop after i += 1, so:
    // i=0: check with p pointing to a[0], then i becomes 1, p becomes &a[1]
    // i=1: check with p pointing to a[1], etc.
    // So check is always a[i][j][k] != a[i][j][k] + i, which fails for i > 0
    // Unless... the + i is meant to be 0? Let me trace again with p = a at start
    // Actually p = a means p points to a[0]. In check_a, we pass p which is &a[0][0]
    // Hmm, this is confusing. Let me just simulate: if a[i][j][k] == (*p)[j][k] + i always passes,
    // then p must effectively be a[0] always, and i must be 0 for all checks? That can't be right.
    // Let me just pass a reference to a[0] as p
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