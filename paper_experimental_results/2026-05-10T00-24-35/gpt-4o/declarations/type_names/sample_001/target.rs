// type_names.rs

fn fip() -> &'static i32 {
    &G_ARR[1]
}

fn f_plain() -> i32 {
    7
}

fn f_var(x: u32, _args: ...) -> i32 {
    (x + 3) as i32
}

fn use_vla(n: usize, p: &[i32]) -> i32 {
    p[0] * 10 + p[n - 1]
}

static G_ARR: [i32; 3] = [1, 2, 3];

fn main() {
    // Check if size of temporary i32 is equal to i32
    {
        if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
            std::process::exit(1);
        }
    }

    // Check dereferencing pointer to first element of G_ARR
    {
        let p: *const i32 = &G_ARR[0];
        if unsafe { *p } != 1 {
            std::process::exit(2);
        }
    }

    // Check array of pointers
    {
        let q: *const i32 = [G_ARR[0].as_ptr(), G_ARR[1].as_ptr(), G_ARR[2].as_ptr()][2];
        if unsafe { *q } != 3 {
            std::process::exit(3);
        }
    }

    // Check array reference
    {
        let pa: &[i32; 3] = &G_ARR;
        if pa[1] != 2 {
            std::process::exit(4);
        }
    }

    // Check use of function with VLA-like behavior
    {
        let n = 3;
        let r = use_vla(n, &G_ARR);
        if r != 13 {
            std::process::exit(5);
        }
    }

    // Check function pointer return
    {
        let pf: fn() -> &'static i32 = fip;
        if *pf() != 2 {
            std::process::exit(6);
        }
    }

    // Check function pointer call
    {
        let pf: fn() -> i32 = f_plain;
        if pf() != 7 {
            std::process::exit(7);
        }
    }

    // Check array of function pointers
    {
        let af: [fn(u32, ...) -> i32; 2] = [f_var, f_var];

        if af[0](4, ()) != 7 {
            std::process::exit(8);
        }

        if af[1](10, (1, 2, 3)) != 13 {
            std::process::exit(9);
        }
    }

    std::process::exit(0);
}