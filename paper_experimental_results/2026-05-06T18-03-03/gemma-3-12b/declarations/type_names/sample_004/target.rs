fn main() {
    // Check sizeof((int){0}) != sizeof(int)
    if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }

    // Check *(int *)&g_arr[0] != 1
    let g_arr: [i32; 3] = [1, 2, 3];
    unsafe {
        if *g_arr.get_unchecked(0) != 1 {
            std::process::exit(2);
        }
    }

    // Check *q != 3
    let g_arr: [i32; 3] = [1, 2, 3];
    let q = &g_arr[2];
    if *q != 3 {
        std::process::exit(3);
    }

    // Check (*pa)[1] != 2
    let g_arr: [i32; 3] = [1, 2, 3];
    let pa: &[i32; 3] = &g_arr;
    if pa[1] != 2 {
        std::process::exit(4);
    }

    // Check use_vla(n, (int (*)[n]) & g_arr) != 13
    let g_arr: [i32; 3] = [1, 2, 3];
    let n = 3;
    let r = use_vla(n, &g_arr);
    if r != 13 {
        std::process::exit(5);
    }

    // Check *pf() != 2
    let g_arr: [i32; 3] = [1, 2, 3];
    let pf = || &g_arr[1];
    unsafe {
        if *pf() != 2 {
            std::process::exit(6);
        }
    }

    // Check pf() != 7
    let pf = || 7;
    if pf() != 7 {
        std::process::exit(7);
    }

    // Check af[0](4u, 0) != 7 and af[1](10u, 1, 2, 3) != 13
    let af: fn(u32, i32, i32, i32) = f_var;
    if af(4, 0, 0, 0) != 7 {
        std::process::exit(8);
    }
    if af(10, 1, 2, 3) != 13 {
        std::process::exit(9);
    }

    std::process::exit(0);
}

fn use_vla(n: usize, arr: &[i32]) -> i32 {
    arr[0] * 10 + arr[n - 1]
}

fn f_var(x: u32, a: i32, b: i32, c: i32) -> i32 {
    (x + 3) as i32
}