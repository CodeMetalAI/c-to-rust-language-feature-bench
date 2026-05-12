fn main() {
    // Check sizeof(int)
    if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }

    // Check g_arr[0]
    let g_arr: [i32; 3] = [1, 2, 3];
    let p = &g_arr[0];
    if *p != 1 {
        std::process::exit(2);
    }

    // Check g_arr[2]
    let q = &g_arr[2];
    if *q != 3 {
        std::process::exit(3);
    }

    // Check g_arr[1]
    let pa: &[[i32; 3]] = &g_arr;
    if pa[0][1] != 2 {
        std::process::exit(4);
    }

    // Check use_vla
    let n = 3;
    let r = use_vla(n, &g_arr);
    if r != 13 {
        std::process::exit(5);
    }

    // Check fip()
    let pf: fn() -> *const i32 = || {
        &g_arr[1]
    };
    if *pf() != 2 {
        std::process::exit(6);
    }

    // Check f_plain()
    let pf: fn() -> i32 = || {
        7
    };
    if pf() != 7 {
        std::process::exit(7);
    }

    // Check f_var()
    let af: [&fn(u32, ...) -> i32; 2] = [f_var, f_var];
    if af[0](4, 0) != 7 {
        std::process::exit(8);
    }
    if af[1](10, 1, 2, 3) != 13 {
        std::process::exit(9);
    }

    std::process::exit(0);
}

fn use_vla(n: usize, p: &[[i32; usize]]) -> i32 {
    let arr = p;
    arr[0][0] * 10 + arr[0][n - 1]
}

fn f_var(x: u32, _...: ()) -> i32 {
    (x + 3) as i32
}

fn f_plain() -> i32 {
    7
}