fn g_arr() -> &'static [i32; 3] {
    static ARR: [i32; 3] = [1, 2, 3];
    &ARR
}

fn fip() -> *const i32 {
    &g_arr()[1]
}

fn f_plain() -> i32 {
    7
}

fn f_var(x: u32) -> i32 {
    (x + 3) as i32
}

fn use_vla(n: usize, p: &[i32]) -> i32 {
    p[0] * 10 + p[n - 1]
}

fn main() {
    {
        let p = &g_arr()[0];
        if p != &1 {
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
        let r = use_vla(n, g_arr());
        if r != 13 {
            std::process::exit(5);
        }
    }

    {
        let pf = fip;
        unsafe {
            if *pf() != 2 {
                std::process::exit(6);
            }
        }
    }

    {
        let pf = f_plain;
        if pf() != 7 {
            std::process::exit(7);
        }
    }

    {
        let af: [fn(u32) -> i32; 2] = [f_var, f_var];

        if af[0](4) != 7 {
            std::process::exit(8);
        }

        if af[1](10) != 13 {
            std::process::exit(9);
        }
    }
}