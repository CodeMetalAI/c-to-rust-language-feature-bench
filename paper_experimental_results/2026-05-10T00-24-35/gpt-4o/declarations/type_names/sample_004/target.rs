fn fip() -> &'static i32 {
    &G_ARR[1]
}

fn f_plain() -> i32 {
    7
}

fn f_var(x: u32, _: ...) -> i32 {
    (x + 3) as i32
}

fn use_vla(p: &[i32]) -> i32 {
    p[0] * 10 + p[p.len() - 1]
}

static G_ARR: [i32; 3] = [1, 2, 3];

fn main() -> i32 {
    // Checking if the size of a temporary int is same as int
    {
        if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
            return 1;
        }
    }

    // Pointer to first element in g_arr
    {
        let p = &G_ARR[0];
        if *p != 1 {
            return 2;
        }
    }

    // Array of pointers and dereferencing
    {
        let q = [&G_ARR[0], &G_ARR[1], &G_ARR[2]][2];
        if *q != 3 {
            return 3;
        }
    }

    // Pointer to array of 3 ints
    {
        let pa: &[i32; 3] = &G_ARR;
        if pa[1] != 2 {
            return 4;
        }
    }

    // Variable length array simulation
    {
        let n = 3;
        let r = use_vla(&G_ARR[0..n]);
        if r != 13 {
            return 5;
        }
    }

    // Function pointer returning a pointer
    {
        let pf = fip as fn() -> &'static i32;
        if *pf() != 2 {
            return 6;
        }
    }

    // Function pointer
    {
        let pf = f_plain as fn() -> i32;
        if pf() != 7 {
            return 7;
        }
    }

    // Array of function pointers
    {
        let af: [fn(u32, ...) -> i32; 2] = [f_var, f_var];

        if af[0](4, std::marker::PhantomData) != 7 {
            return 8;
        }

        if af[1](10, std::marker::PhantomData) != 13 {
            return 9;
        }
    }

    0
}