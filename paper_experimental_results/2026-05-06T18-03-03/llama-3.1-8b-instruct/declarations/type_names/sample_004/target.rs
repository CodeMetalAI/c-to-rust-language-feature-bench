fn main() {
    let g_arr = [1, 2, 3];

    let fip = || g_arr[1];

    let f_plain = || 7;

    let f_var = |x: u32, ...| -> i32 { x as i32 + 3 };

    let use_vla = |n: usize, p: &[[i32; n]]| -> i32 {
        p[0][0] * 10 + p[n - 1][0]
    };

    {
        if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
            std::process::exit(1);
        }
    }

    {
        let p = &g_arr[0] as *const i32;
        if *p != 1 {
            std::process::exit(2);
        }
    }

    {
        let q = [&g_arr[0], &g_arr[1], &g_arr[2]][2];
        if *q != 3 {
            std::process::exit(3);
        }
    }

    {
        let pa: &[[i32; 3]] = &g_arr;
        if pa[1][0] != 2 {
            std::process::exit(4);
        }
    }

    {
        let n = 3;
        let r = use_vla(n, &g_arr);
        if r != 13 {
            std::process::exit(5);
        }
    }

    {
        let pf = fip;
        if pf() != 2 {
            std::process::exit(6);
        }
    }

    {
        let pf = f_plain;
        if pf() != 7 {
            std::process::exit(7);
        }
    }

    {
        let af = [f_var, f_var];
        if af[0](4) != 7 {
            std::process::exit(8);
        }

        if af[1](10, 1, 2, 3) != 13 {
            std::process::exit(9);
        }
    }

    std::process::exit(0);
}