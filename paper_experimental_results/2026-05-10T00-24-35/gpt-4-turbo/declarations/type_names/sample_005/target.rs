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

fn f_var(x: u32, _args: &mut std::slice::Iter<u32>) -> i32 {
    x as i32 + 3
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
        let p = g_arr().as_ptr();
        if unsafe { *p } != 1 {
            std::process::exit(2);
        }
    }

    {
        let q = &[g_arr()[0], g_arr()[1], g_arr()[2]];
        if q[2] != 3 {
            std::process::exit(3);
        }
    }

    {
        let pa = g_arr();
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
        let pf = fip as fn() -> *const i32;
        if unsafe { *pf() } != 2 {
            std::process::exit(6);
        }
    }

    {
        let pf = f_plain as fn() -> i32;
        if pf() != 7 {
            std::process::exit(7);
        }
    }

    {
        let funcs: [fn(u32, &mut std::slice::Iter<u32>) -> i32; 2] = [f_var, f_var];
        let mut args = std::slice::from_ref(&0u32).iter();
        if funcs[0](4, &mut args) != 7 {
            std::process::exit(8);
        }

        let mut args = vec![1u32, 2, 3].iter();
        if funcs[1](10, &mut args) != 13 {
            std::process::exit(9);
        }
    }
}