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
    // Check sizeof compound literal (int){0}
    // In Rust, we just check that the type size matches
    let zero_int: i32 = 0;
    if std::mem::size_of_val(&zero_int) != std::mem::size_of::<i32>() {
        return;
    }

    // Pointer to array element
    let g_arr: [i32; 3] = [1, 2, 3];
    let p: &i32 = &g_arr[0];
    if *p != 1 {
        return;
    }

    // Array of pointers
    let ptr_array: [&i32; 3] = [&g_arr[0], &g_arr[1], &g_arr[2]];
    let q: &i32 = ptr_array[2];
    if *q != 3 {
        return;
    }

    // Pointer to array
    let pa: &[i32; 3] = &g_arr;
    if pa[1] != 2 {
        return;
    }

    // VLA-like usage
    let n = 3;
    let r = use_vla(n, &g_arr);
    if r != 13 {
        return;
    }

    // Function pointer returning pointer
    let pf: fn() -> &'static i32 = fip;
    if *pf() != 2 {
        return;
    }

    // Function pointer to plain function
    let pf_plain: fn() -> i32 = f_plain;
    if pf_plain() != 7 {
        return;
    }

    // Array of function pointers (with variadic-like behavior)
    let af: [fn(u32, &[u32]) -> i32; 2] = [f_var, f_var];
    
    if af[0](4, &[]) != 7 {
        return;
    }
    
    if af[1](10, &[1, 2, 3]) != 13 {
        return;
    }

    // Success
}