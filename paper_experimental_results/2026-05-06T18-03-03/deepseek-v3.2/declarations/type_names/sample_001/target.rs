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
    // Test 1: sizeof compound literal
    {
        let temp: i32 = 0;
        if std::mem::size_of_val(&temp) != std::mem::size_of::<i32>() {
            std::process::exit(1);
        }
    }

    // Test 2: pointer to array element
    {
        static G_ARR: [i32; 3] = [1, 2, 3];
        let p: &i32 = &G_ARR[0];
        if *p != 1 {
            std::process::exit(2);
        }
    }

    // Test 3: array of pointers
    {
        static G_ARR: [i32; 3] = [1, 2, 3];
        let arr: [&i32; 3] = [&G_ARR[0], &G_ARR[1], &G_ARR[2]];
        let q: &i32 = arr[2];
        if *q != 3 {
            std::process::exit(3);
        }
    }

    // Test 4: pointer to array
    {
        static G_ARR: [i32; 3] = [1, 2, 3];
        let pa: &[i32; 3] = &G_ARR;
        if pa[1] != 2 {
            std::process::exit(4);
        }
    }

    // Test 5: variable length array simulation
    {
        static G_ARR: [i32; 3] = [1, 2, 3];
        let n = 3;
        let r = use_vla(n, &G_ARR);
        if r != 13 {
            std::process::exit(5);
        }
    }

    // Test 6: function pointer returning pointer
    {
        let pf: fn() -> &'static i32 = fip;
        if *pf() != 2 {
            std::process::exit(6);
        }
    }

    // Test 7: plain function pointer
    {
        let pf: fn() -> i32 = f_plain;
        if pf() != 7 {
            std::process::exit(7);
        }
    }

    // Test 8/9: array of function pointers with variadic simulation
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