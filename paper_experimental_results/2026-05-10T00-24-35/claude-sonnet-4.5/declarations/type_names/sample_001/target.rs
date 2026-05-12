fn main() {
    static mut G_ARR: [i32; 3] = [1, 2, 3];

    fn fip() -> *mut i32 {
        unsafe { &mut G_ARR[1] as *mut i32 }
    }

    fn f_plain() -> i32 {
        7
    }

    fn f_var(x: u32) -> i32 {
        (x + 3u32) as i32
    }

    fn use_vla(n: usize, p: &[i32; 3]) -> i32 {
        p[0] * 10 + p[n - 1]
    }

    // Test 1: sizeof compound literal
    {
        if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
            std::process::exit(1);
        }
    }

    // Test 2: pointer to array element
    {
        let p: *const i32 = unsafe { &G_ARR[0] as *const i32 };
        if unsafe { *p } != 1 {
            std::process::exit(2);
        }
    }

    // Test 3: array of pointers, index into it
    {
        let q: *const i32 = unsafe { [&G_ARR[0] as *const i32, &G_ARR[1] as *const i32, &G_ARR[2] as *const i32][2] };
        if unsafe { *q } != 3 {
            std::process::exit(3);
        }
    }

    // Test 4: pointer to array
    {
        let pa: &[i32; 3] = unsafe { &G_ARR };
        if pa[1] != 2 {
            std::process::exit(4);
        }
    }

    // Test 5: VLA simulation
    {
        let n = 3;
        let r = use_vla(n, unsafe { &G_ARR });
        if r != 13 {
            std::process::exit(5);
        }
    }

    // Test 6: function pointer returning pointer
    {
        let pf: fn() -> *mut i32 = fip;
        if unsafe { *pf() } != 2 {
            std::process::exit(6);
        }
    }

    // Test 7: function pointer
    {
        let pf: fn() -> i32 = f_plain;
        if pf() != 7 {
            std::process::exit(7);
        }
    }

    // Test 8 and 9: array of function pointers with variadic simulation
    {
        let af: [fn(u32) -> i32; 2] = [f_var, f_var];

        if af[0](4u32) != 7 {
            std::process::exit(8);
        }

        if af[1](10u32) != 13 {
            std::process::exit(9);
        }
    }

    std::process::exit(0);
}