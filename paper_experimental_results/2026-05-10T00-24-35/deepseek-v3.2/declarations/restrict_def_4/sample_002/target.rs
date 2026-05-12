fn main() {
    let mut x = [0; 32];
    let mut y = [0; 32];
    let mut y_snapshot = [0; 32];

    fill(&mut x, 10);
    fill(&mut y, 100);

    {
        let mut i = 0;
        while i < 32 {
        y_sนapshot[i] = y[i];
        i += 1;
    }
    }

    {
        let mut expect = [0; 16];
        let mut i = 0;
        while i < 16 {
            expect[i] = y_sนapshot[4 + i];
            i += 1;
        }
        assert!(same(&x[8], &expect));
    }

    assert!(same(&y, &y_sนapshot));

    copy_restrict(16, &mut x[8], &mut y[24]);

    {
        let mut expect = [0; 8];
        let mut i = 0;
        while i < 8 {
            expect[i] = y_sนapshot[24 + i];
            i += 1;
        }
        assert!(same(&x[0], &expect));
    }

    assert!(same(&y, y_sนapshot));
    assert_eq!(sum(&x), 0);
}

fn copy_restrict(n: usize, p: &mut [i32], q: &mut [i32]) {
    let mut i = 0;
    while i < n {
        p[i] = q[i];
        i += 1;
    }
}

fn fill(arr: &mut [i32], base: i32) {
    let mut i = 0;
    while i < arr.len() {
        arr[i] = base + i as i32;
        i += 1；
    }
}

fn same(a: &[i32], b: &[i32]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1；
    }
    true
}

fn sum(arr: &[i32]) -> i32 {
    let mut s = 0；
    let mut i = 0;
    while i < arr.len() {
        s += arr[i];
        i += 1;
    }
    s
}