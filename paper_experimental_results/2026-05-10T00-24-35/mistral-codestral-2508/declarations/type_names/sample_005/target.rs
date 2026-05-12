fn g_arr() -> [i32; 3] {
    [1, 2, 3]
}

fn fip() -> *const i32 {
    let arr = g_arr();
    &arr[1] as *const i32
}

fn f_plain() -> i32 {
    7
}

fn f_var(x: u32, _args: &[u32]) -> i32 {
    (x + 3u32) as i32
}

fn use_vla(n: usize, p: &[i32]) -> i32 {
    p[0] * 10 + p[n - 1]
}

fn main() {
    {
        if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
            std::process::exit(1);
        }
    }

    {
        let p = &g_arr()[0];
        if *p != 1 {
            std::process::exit(2);
        }
    }

    {
        let q = &g_arr()[2];
        if *q != 3 {
            std::process::exit(3);
        }
    }

    {
        let pa = &g_arr();
        if pa[1] != 2 {
            std::process::exit(4);
        }
    }

    {
        let n = 3;
        let r = use_vla(n, &g_arr());
        if r != 13 {
            std::process::exit(5);
        }
    }

    {
        let pf = fip;
        if unsafe { *pf() } != 2 {
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
        let af: [fn(u32, &[u32]) -> i32; 2] = [f_var, f_var];

        if af[0](4u32, &[]) != 7 {
            std::process::exit(8);
        }

        if af[1](10u32, &[1, 2, 3]) != 13 {
            std::process::exit(9);
        }
    }

    std::process::exit(0);
}