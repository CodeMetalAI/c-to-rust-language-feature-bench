fn fip() -> &'static i32 {
    static G_ARR: [i32; 3] = [1, 2, 3];
    &G_ARR[1]
}

fn f_plain() -> i32 {
    7
}

fn f_var(x: u32, _args: &[u32]) -> i32 {
    (x + 3) as i32
}

fn use_vla(n: usize, p: &[i32]) -> i32 {
    p[0] * 10 + p[n - 1]
}

fn main() {
    // Check sizeof((int){0}) - compound literal not in Rust, but we can check size of i32
    {
        if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
            std::process::exit(1);
        }
    }

    // Pointer to array element
    {
        static G_ARR: [i32; 3] = [1, 2, 3];
        let p: &i32 = &G_ARR[0];
        if *p != 1 {
            std::process::exit(2);
        }
    }

    // Array of pointers
    {
        static G_ARR: [i32; 3] = [1, 2, 3];
        let arr: [&i32; 3] = [&G_ARR[0], &G_ARR[1], &G_ARR[2]];
        let q: &i32 = arr[2];
        if **q != 3 {
            std::process::exit(3);
        }
    }

    // Pointer to array
    {
        static G_ARR: [i32; 3] = [1, 2, 3];
        let pa: &[i32; 3] = &G_ARR;
        if pa[1] != 2 {
            std::process::exit(4);
        }
    }

    // VLA-like usage
    {
        static G_ARR: [i32; 3] = [1, 2, 3];
        let n = 3;
        let r = use_vla(n, &G_ARR);
        if r != 13 {
            std::process::exit(5);
        }
    }

    // Function pointer
    {
        let pf: fn() -> &'static i32 = fip;
        if *pf() != 2 {
            std::process::exit(6);
        }
    }

    // Plain function pointer
    {
        let pf: fn() -> i32 = f_plain;
        if pf() != 7 {
            std::process::exit(7);
        }
    }

    // Array of function pointers with varargs simulation
    {
        let af: [fn(u32, &[u32]) -> i32; 2] = [f_var, f_var];
        
        if af[0](4, &[]) != 7 {
            std::process::exit(8);
        }
        
        if af[1](10, &[1, 2, 3]) != 13 {
            std::process::exit(9);
        }
    }

    std::process::exit(0);
}