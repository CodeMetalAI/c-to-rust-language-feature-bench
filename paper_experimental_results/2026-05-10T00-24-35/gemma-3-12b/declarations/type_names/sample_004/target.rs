fn main() {
    // Check sizeof(int)
    if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }

    // Check g_arr[0]
    let g_arr: [i32; 3] = [1, 2, 3];
    unsafe {
        let p = &g_arr[0] as *const i32;
        if *p != 1 {
            std::process::exit(2);
        }
    }

    // Check g_arr[2]
    unsafe {
        let q = &g_arr[2] as *const i32;
        if *q != 3 {
            std::process::exit(3);
        }
    }

    // Check (*pa)[1]
    unsafe {
        let pa: *const [i32; 3] = &g_arr as *const [i32; 3];
        if (*pa)[1] != 2 {
            std::process::exit(4);
        }
    }

    // Check use_vla
    let n = 3;
    unsafe {
        let pa: *const [i32; 3] = &g_arr as *const [i32; 3];
        let r = use_vla(n, &*pa);
        if r != 13 {
            std::process::exit(5);
        }
    }

    // Check fip()
    unsafe {
        let pf: fn() -> *const i32 = fip;
        if *pf() != 2 {
            std::process::exit(6);
        }
    }

    // Check f_plain()
    let pf: fn() -> i32 = f_plain;
    if pf() != 7 {
        std::process::exit(7);
    }

    // Check f_var()
    let af: [&fn(u32, ...); 2] = [f_var, f_var];
    if af[0](4, 0) != 7 {
        std::process::exit(8);
    }
    if af[1](10, 1, 2, 3) != 13 {
        std::process::exit(9);
    }

    std::process::exit(0);
}

fn fip() -> *const i32 {
    &[1, 2, 3][1]
}

static fn f_plain() -> i32 {
    7
}

static fn f_var(x: u32, _...: ()) -> i32 {
    (x + 3) as i32
}

unsafe fn use_vla(n: i32, p: *const [i32; 3]) -> i32 {
    let p_slice = std::slice::from_raw_parts(p, n as usize);
    p_slice[0] * 10 + p_slice[n as usize - 1]
}