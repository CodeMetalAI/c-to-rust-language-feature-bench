fn main() {
    if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }

    let g_arr: [i32; 3] = [1, 2, 3];
    let p = &g_arr[0];
    if *p != 1 {
        std::process::exit(2);
    }

    let q = &g_arr[2];
    if *q != 3 {
        std::process::exit(3);
    }

    let pa: &[[i32; 3]] = &g_arr;
    if pa[1] != 2 {
        std::process::exit(4);
    }

    let n = 3;
    let r = use_vla(n, &g_arr);
    if r != 13 {
        std::process::exit(5);
    }

    let pf: fn() -> *const i32 = || fip();
    if *pf() != 2 {
        std::process::exit(6);
    }

    let f_plain_ptr: fn() -> i32 = f_plain;
    if f_plain_ptr() != 7 {
        std::process::exit(7);
    }

    let af: &[fn(u32, ...) -> i32] = &[f_var, f_var];

    if af[0](4, 0) != 7 {
        std::process::exit(8);
    }

    if af[1](10, 1, 2, 3) != 13 {
        std::process::exit(9);
    }

    std::process::exit(0);
}

fn fip() -> *const i32 {
    &[1, 2, 3][1]
}

fn f_plain() -> i32 {
    7
}

fn f_var(x: u32, ...) -> i32 {
    (x + 3) as i32
}

fn use_vla(n: usize, arr: &[[i32; 3]]) -> i32 {
    arr[0][0] * 10 + arr[0][n - 1]
}