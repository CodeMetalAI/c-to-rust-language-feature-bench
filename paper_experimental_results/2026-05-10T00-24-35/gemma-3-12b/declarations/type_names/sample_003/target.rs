fn main() {
    // {
    //     if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
    //         return 1;
    //     }
    // }

    let g_arr: [i32; 3] = [1, 2, 3];

    // {
    //     let p = &g_arr[0];
    //     if *p != 1 {
    //         return 2;
    //     }
    // }

    // {
    //     let q = &g_arr[2];
    //     if *q != 3 {
    //         return 3;
    //     }
    // }

    // {
    //     let pa: &[[i32; 3]] = &g_arr;
    //     if pa[1][0] != 2 {
    //         return 4;
    //     }
    // }

    // {
    //     let n = 3;
    //     let r = use_vla(n, &g_arr);
    //     if r != 13 {
    //         return 5;
    //     }
    // }

    fn fip() -> *mut i32 {
        &g_arr[1]
    }

    // {
    //     let pf: fn() -> *mut i32 = fip;
    //     if *pf() != 2 {
    //         return 6;
    //     }
    // }

    fn f_plain() -> i32 {
        7
    }

    // {
    //     let pf: fn() -> i32 = f_plain;
    //     if pf() != 7 {
    //         return 7;
    //     }
    // }

    fn f_var(x: u32, ...) -> i32 {
        (x + 3) as i32
    }

    // {
    //     let af: [&fn(u32, ...)] = [f_var, f_var];
    //     if af[0](4, 0) != 7 {
    //         return 8;
    //     }
    //     if af[1](10, 1, 2, 3) != 13 {
    //         return 9;
    //     }
    // }

    fn use_vla(n: usize, arr: &[i32]) -> i32 {
        arr[0] * 10 + arr[n - 1]
    }

    0
}