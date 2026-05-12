fn fip() -> *const i32 {
    static G_ARR: [i32; 3] = [1, 2, 3];
    unsafe { &G_ARR[1] }
}

fn f_plain() -> i32 {
    7
}

fn f_var(x: u32, _: ...) -> i32 {
    (x + 3) as i32
}

fn use_vla(n: usize, p: *const [i32]) -> i32 {
    unsafe {
        let slice = &*p;
        slice[0] * 10 + slice[n - 1]
    }
}

fn main() {
    static G_ARR: [i32; 3] = [1, 2, 3];

    // Test 1: sizeof((int){0}) != sizeof(int)
    {
        let x: i32 = 0;
        if std::mem::size_of_val(&x) != std::mem::size_of::<i32>() {
            std::process::exit(1);
        }
    }

    // Test 2: int *p = (int *)&g_arr[0]; if (*(int *)(p) != 1) return 2;
    {
        let p: *const i32 = &G_ARR[0];
        unsafe {
            if *p != 1 {
                std::process::exit(2);
            }
        }
    }

    // Test 3: int *q = (int *[3]){&g_arr[0], &g_arr[1], &g_arr[2]}[2]; if (*q != 3) return 3;
    {
        let ptrs: [*const i32; 3] = [&G_ARR[0], &G_ARR[1], &G_ARR[2]];
        let q = ptrs[2];
        unsafe {
            if *q != 3 {
                std::process::exit(3);
            }
        }
    }

    // Test 4: int (*pa)[3] = (int (*)[3]) & g_arr; if ((*pa)[1] != 2) return 4;
    {
        let pa: *const [i32; 3] = &G_ARR;
        unsafe {
            if (*pa)[1] != 2 {
                std::process::exit(4);
            }
        }
    }

    // Test 5: int n = 3; int r = use_vla(n, (int (*)[n]) & g_arr); if (r != 13) return 5;
    {
        let n = 3;
        let r = use_vla(n, &G_ARR as *const [i32; 3] as *const [i32]);
        if r != 13 {
            std::process::exit(5);
        }
    }

    // Test 6: int *(*pf)() = (int *(*)())fip; if (*pf() != 2) return 6;
    {
        let pf: fn() -> *const i32 = fip;
        unsafe {
            if *pf() != 2 {
                std::process::exit(6);
            }
        }
    }

    // Test 7: int (*pf)(void) = (int (*)(void))f_plain; if (pf() != 7) return 7;
    {
        let pf: fn() -> i32 = f_plain;
        if pf() != 7 {
            std::process::exit(7);
        }
    }

    // Test 8 & 9: int (*const af[])(unsigned int, ...) = {f_var, f_var};
    {
        let af: [fn(u32, ...) -> i32; 2] = [f_var, f_var];
        if af[0](4u32, 0) != 7 {
            std::process::exit(8);
        }
        if af[1](10u32, 1, 2, 3) != 13 {
            std::process::exit(9);
        }
    }

    std::process::exit(0);
}