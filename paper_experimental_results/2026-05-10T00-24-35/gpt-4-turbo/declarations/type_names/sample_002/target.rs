fn g_arr() -> &'static [i32; 3] {
    &mut [1, 2, 3]
}

fn fip() -> *const i32 {
    g_arr().as_ptr().add(1)
}

fn f_plain() -> i32 {
    7
}

fn f_var(x: u32, _: &()) -> i32 {
    (x + 3) as i32
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
        let q = [g_arr().as_ptr(), g_arr().as_ptr().add(1), g_arr().as_ptr().add(2)][2];
        if unsafe { *q } != 3 {
            std::process::exit(3);
        }
    }

    {
        let pa = g_arr() as *const i32 as *const [i32; 3];
        if unsafe { (*pa)[1] } != 2 {
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
        let af: [fn(u32, &()) -> i32; 2] = [f_var, f_var];
        if af[0](4, &()) != 7 {
            std::process::exit(8);
        }

        if af[1](10, &()) != 13 {
            std::process::exit(9);
        }
    }
}