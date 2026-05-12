// type_names.rs

fn fip() -> &'static i32 {
    &G_ARR[1]
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

static G_ARR: [i32; 3] = [1, 2, 3];

fn main() {
    {
        if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
            std::process::exit(1);
        }
    }

    {
        let p: *const i32 = &G_ARR[0];
        if unsafe { *p } != 1 {
            std::process::exit(2);
        }
    }

    {
        let q: *const i32 = [&G_ARR[0], &G_ARR[1], &G_ARR[2]][2];
        if unsafe { *q } != 3 {
            std::process::exit(3);
        }
    }

    {
        let pa: &[i32; 3] = &G_ARR;
        if pa[1] != 2 {
            std::process::exit(4);
        }
    }

    {
        let n = 3;
        let r = use_vla(&G_ARR[..n]);
        if r != 13 {
            std::process::exit(5);
        }
    }

    {
        let pf: fn() -> &'static i32 = fip;
        if *pf() != 2 {
            std::process::exit(6);
        }
    }

    {
        let pf: fn() -> i32 = f_plain;
        if pf() != 7 {
            std::process::exit(7);
        }
    }

    {
        let af: [fn(u32, &[i32]) -> i32; 2] = [f_var, f_var];

        if af[0](4, &[]) != 7 {
            std::process::exit(8);
        }

        if af[1](10, &[1, 2, 3]) != 13 {
            std::process::exit(9);
        }
    }

    std::process::exit(0);
}