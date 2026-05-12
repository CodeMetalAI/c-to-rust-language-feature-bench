use std::mem::size_of;
use std::process::exit;

static G_ARR: [i32; 3] = [1, 2, 3];

fn fip() -> &'static i32 {
    &G_ARR[1]
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
    if size_of::<i32>() != size_of::<i32>() {
        exit(1);
    }

    if G_ARR[0] != 1 {
        exit(2);
    }

    if G_ARR[2] != 3 {
        exit(3);
    }

    if G_ARR[1] != 2 {
        exit(4);
    }

    {
        let n = 3usize;
        let r = use_vla(n, &G_ARR);
        if r != 13 {
            exit(5);
        }
    }

    {
        let pf: fn() -> &'static i32 = fip;
        if *pf() != 2 {
            exit(6);
        }
    }

    {
        let pf: fn() -> i32 = f_plain;
        if pf() != 7 {
            exit(7);
        }
    }

    {
        let af: [fn(u32) -> i32; 2] = [f_var, f_var];

        if af[0](4) != 7 {
            exit(8);
        }

        if af[1](10) != 13 {
            exit(9);
        }
    }

    exit(0);
}