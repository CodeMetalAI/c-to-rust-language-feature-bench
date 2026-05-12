fn main() {
    let mut gate4 = 1;

    let mut x: [i32; 32] = [0; 32];
    let mut y: [i32; 32] = [0; 32];
    let mut y_snapshot: [i32; 32] = [0; 32];

    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);

    for i in 0..32 {
        y_snapshot[i] = y[i];
    }

    let mut p1 = &mut x;
    let mut q1 = &mut y;

    if gate4!= 0 {
        let mut p2 = &mut x[8..];
        let mut q2 = &mut y[4..];

        copy_restrict(p2, q2, 16);

        let expect: [i32; 16] = y_snapshot[4..20].try_into().unwrap();
        if!same(&x[8..24], &expect) {
            std::process::exit(1);
        }

        if!same(&y, &y_snapshot) {
            std::process::exit(2);
        }
    }

    let mut p1 = &mut x[0..];
    let mut q1 = &mut y[24..];

    copy_restrict(p1, q1, 8);

    let expect2: [i32; 8] = y_snapshot[24..32].try_into().unwrap();
    if!same(&x[0..8], &expect2) {
        std::process::exit(3);
    }

    if!same(&y, &y_snapshot) {
        std::process::exit(4);
    }

    if sum(&x) == 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}

fn copy_restrict(dst: &mut [i32], src: &mut [i32], n: usize) {
    for i in 0..n {
        dst[i] = src[i];
    }
}

fn fill(arr: &mut [i32], n: usize, base: i32) {
    for i in 0..n {
        arr[i] = base + i as i32 * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32], n: usize) -> bool {
    a.iter().zip(b.iter()).take(n).all(|(x, y)| x == y)
}

fn sum(arr: &[i32]) -> i32 {
    arr.iter().sum()
}