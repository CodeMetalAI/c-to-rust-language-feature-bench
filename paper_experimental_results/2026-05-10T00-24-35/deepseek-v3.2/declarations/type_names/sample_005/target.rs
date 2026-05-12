fn f_var(x: u32, _: ...) -> i32 {
    (x + 3) as i32
}

fn main() {
    // Test 1: sizeof compound literal
    if std::mem::size_of_val(&0_i32) != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }

    let g_arr = [1, 2, 3];

    // Test 2: pointer to array element
    {
        let p = &g_arr[0] as *const i32;
        if unsafe { *p } != 1 {
            std::process::exit(2);
        }
    }

    // Test 3: array of pointers
    {
        let ptr_array = [&g_arr[0], &g_arr[1], &g_arr[2]];
        let q = ptr_array[2];
        if *q != 3 {
            std::process::exit(3);
        }
    }

    // Test 4: pointer to array
    {
        let pa = &g_arr as *const [i32; 3];
        if unsafe { (*pa)[1] } != 2 {
            std::process::exit(4);
        }
    }

    // Test 5: variable-length array (simulated with slice)
    {
        fn use_vla(n: usize, p: &[i32]) -> i32 {
            p[0] * 10 + p[n - 1]
        }
        let n = 3;
        let r = use_vla(n, &g_arr);
        if r != 13 {
            std::process::exit(5);
        }
    }

    // Test 6: function pointer returning pointer
    {
        fn fip() -> *const i32 {
            &g_arr[1]
        }
        let pf: fn() -> *const i32 = fip;
        if unsafe { *pf() } != 2 {
            std::process::exit(6);
        }
    }

    // Test 7: function pointer to plain function
    {
        fn f_plain() -> i32 {
            7
        }
        let pf: fn() -> i32 = f_plain;
        if pf() != 7 {
            std::process::exit(7);
        }
    }

    // Test 8: array of function pointers with variadic functions
    // Note: Rust doesn't support C-style variadic functions in safe code,
    // so we'll use a workaround with fixed arguments
    {
        let af: [fn(u32, ...) -> i32; 2] = [f_var, f_var];
        if af[0](4, 0) != 7 {
            std::process::exit(8);
        }
        if af[1](10, 1, 2, 3) != 13 {
            std::process::exit(9);
        }
    }

    std::process::exit(0);
}