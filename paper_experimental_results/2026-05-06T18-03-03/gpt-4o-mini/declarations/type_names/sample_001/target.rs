// type_names.rs
static G_ARR: [i32; 3] = [1, 2, 3];

fn fip() -> &'static i32 {
    &G_ARR[1]
}

fn f_plain() -> i32 {
    7
}

fn f_var(x: u32, _: ...) -> i32 {
    (x + 3) as i32
}

fn use_vla(n: usize, p: &[i32; 3]) -> i32 {
    p[0] * 10 + p[n - 1]
}

fn main() -> i32 {
    {
        if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
            return 1;
        }
    }

    {
        let p: *const i32 = &G_ARR[0] as *const i32;
        if unsafe { *p } != 1 {
            return 2;
        }
    }

    {
        let q: &i32 = &G_ARR[2];
        if *q != 3 {
            return 3;
        }
    }

    {
        let pa: &[i32; 3] = &G_ARR;
        if pa[1] != 2 {
            return 4;
        }
    }

    {
        let n = 3;
        let r = use_vla(n, &G_ARR);
        if r != 13 {
            return 5;
        }
    }

    {
        let pf: fn() -> &i32 = fip;
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
        let af: [&dyn Fn(u32, ...) -> i32; 2] = [f_var, f_var];

        if af[0](4, 0) != 7 {
            return 8;
        }

        if af[1](10, 1, 2, 3) != 13 {
            return 9;
        }
    }

    0
}