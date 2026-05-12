fn fip() -> *const i32 {
    static G_ARR: [i32; 3] = [1, 2, 3];
    &G_ARR[1]
}

fn f_plain() -> i32 {
    7
}

fn f_var(x: u32, _args: &[i32]) -> i32 {
    (x + 3) as i32
}

fn use_vla(n: usize, p: &[i32]) -> i32 {
    p[0] * 10 + p[n - 1]
}

fn main() {
    let g_arr: [i32; 3] = [1, 2, 3];
    // Check compound literal sizing
    assert_eq!(std::mem::size_of::<i32>(), std::mem::size_of::<i32>());

    // Pointer to first element
    let p = &g_arr[0] as *const i32;
    assert_eq!(unsafe { *p }, 1);

    // Array of pointers to elements of g_arr
    let q = &[&g_arr[0], &g_arr[1], &g_arr[2]];
    assert_eq!(*q[2], 3);

    // Pointer to array of 3 integers
    let pa = &g_arr as *const [i32; 3];
    assert_eq!(unsafe { (*pa)[1] }, 2);

    // Using a variable length array (VLA) emulation
    let n = 3;
    let r = use_vla(n, &g_arr);
    if r != 13 {
        std::process::exit(5);
    }

    // Function pointers
    let pf = fip;
    assert_eq!(unsafe { *pf() }, 2);

    let pf_plain: fn() -> i32 = f_plain;
    assert_eq!(pf_plain(), 7);

    // Array of function pointers
    let af: [fn(u32, &[i32]) -> i32; 2] = [f_var, f_var];
    if af[0](4, &[]) != 7 {
        std::process::exit(8);
    }

    if af[1](10, &[1, 2, 3]) != 13 {
        std::process::exit(9);
    }
}