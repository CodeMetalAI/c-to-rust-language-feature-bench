static G_ARR: [i32; 3] = [1, 2, 3];

fn fip() -> *const i32 {
    &G_ARR[1]
}

fn f_plain() -> i32 {
    7
}

fn f_var(x: u32, _: ...) -> i32 {
    (x + 3) as i32
}

fn use_vla(n: usize, p: &[i32]) -> i32 {
    p[0] * 10 + p[n - 1]
}

fn main() {
    // Checking the size of an integer literal
    {
        if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
            std::process::exit(1);
        }
    }

    // Checking a pointer (reference) to the first element
    {
        let p: *const i32 = &G_ARR[0];
        if unsafe { *p } != 1 {
            std::process::exit(2);
        }
    }

    // Checking array of references to elements
    {
        let arr = [&G_ARR[0], &G_ARR[1], &G_ARR[2]];
        let q = arr[2];
        if *q != 3 {
            std::process::exit(3);
        }
    }

    // Checking a reference to the array
    {
        let pa: &[i32; 3] = &G_ARR;
        if pa[1] != 2 {
            std::process::exit(4);
        }
    }

    // Using a slice to simulate VLA
    {
        let n = 3;
        let r = use_vla(n, &G_ARR[..n]);
        if r != 13 {
            std::process::exit(5);
        }
    }

    // Function pointer usage
    {
        let pf: fn() -> *const i32 = fip;
        if unsafe { *pf() } != 2 {
            std::process::exit(6);
        }
    }

    // Pointer to a function returning an integer
    {
        let pf: fn() -> i32 = f_plain;
        if pf() != 7 {
            std::process::exit(7);
        }
    }

    // Array of function pointers with variadic function
    {
        let af: [fn(u32, ...) -> i32; 2] = [f_var, f_var];

        if af[0](4, ()) != 7 {
            std::process::exit(8);
        }

        if af[1](10, ()) != 13 {
            std::process::exit(9);
        }
    }

    std::process::exit(0);
}