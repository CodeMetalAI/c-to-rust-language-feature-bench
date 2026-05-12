fn g_arr() -> &'static [i32; 3] {
    &[1, 2, 3]
}

fn fip() -> &'static i32 {
    &g_arr()[1]
}

fn f_plain() -> i32 {
    7
}

fn f_var(x: u32, _args: &[i32]) -> i32 {
    (x + 3) as i32
}

fn use_vla(p: &[i32]) -> i32 {
    p[0] * 10 + p[p.len() - 1]
}

fn main() -> i32 {
    {
        // Equivalent of compound literal (int){0} in C
        if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
            return 1;
        }
    }

    {
        let p = &g_arr()[0] as *const i32;
        if unsafe { *p } != 1 {
            return 2;
        }
    }

    {
        let q = [&g_arr()[0], &g_arr()[1], &g_arr()[2]][2];
        if *q != 3 {
            return 3;
        }
    }

    {
        let pa = g_arr();
        if pa[1] != 2 {
            return 4;
        }
    }

    {
        let r = use_vla(&g_arr()[..]);
        if r != 13 {
            return 5;
        }
    }

    {
        let pf: fn() -> &'static i32 = fip;
        if *pf() != 2 {
            return 6;
        }
    }

    {
        let pf: fn() -> i32 = f_plain;
        if pf() != 7 {
            return 7;
        }
    }

    {
        let af: [fn(u32, &[i32]) -> i32; 2] = [f_var, f_var];

        if af[0](4, &[]) != 7 {
            return 8;
        }

        if af[1](10, &[1, 2, 3]) != 13 {
            return 9;
        }
    }

    0
}