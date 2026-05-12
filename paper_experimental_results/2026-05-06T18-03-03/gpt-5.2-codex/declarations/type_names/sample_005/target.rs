use std::process::exit;

static G_ARR: [i32; 3] = [1, 2, 3];

fn fip() -> &'static i32 {
    &G_ARR[1]
}

fn f_plain() -> i32 {
    7
}

fn f_var(x: u32, _args: &[u32]) -> i32 {
    (x + 3) as i32
}

fn use_vla(n: usize, p: &[i32]) -> i32 {
    p[0] * 10 + p[n - 1]
}

fn main() {
    // Block 1
    if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
        exit(1);
    }

    // Block 2
    let p = &G_ARR[0];
    if *p != 1 {
        exit(2);
    }

    // Block 3
    let q = [&G_ARR[0], &G_ARR[1], &G_ARR[2]][2];
    if *q != 3 {
        exit(3);
    }

    // Block 4
    let pa: &[i32; 3] = &G_ARR;
    if pa[1] != 2 {
        exit(4);
    }

    // Block 5
    let n = 3usize;
    let r = use_vla(n, &G_ARR);
    if r != 13 {
        exit(5);
    }

    // Block 6
    let pf: fn() -> &'static i32 = fip;
    if *pf() != 2 {
        exit(6);
    }

    // Block 7
    let pf_plain: fn() -> i32 = f_plain;
    if pf_plain() != 7 {
        exit(7);
    }

    // Block 8
    let af: [fn(u32, &[u32]) -> i32; 2] = [f_var, f_var];
    if af[0](4u32, &[0]) != 7 {
        exit(8);
    }
    if af[1](10u32, &[1, 2, 3]) != 13 {
        exit(9);
    }

    exit(0);
}