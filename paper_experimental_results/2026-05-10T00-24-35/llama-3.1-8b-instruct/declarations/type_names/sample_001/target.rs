fn main() {
    let g_arr = [1, 2, 3];

    let g_arr_ref = &g_arr[1];

    static F_PLAIN: fn() -> i32 = || 7;

    static mut F_VAR: fn(u32, ...) -> i32 = |x, x_| (x + 3) as i32;

    let n = 3;
    let g_arr_slice = unsafe { &g_arr[..] };
    let result = use_vla(n, &g_arr_slice);

    let fip = || g_arr_ref;

    let f_plain = F_PLAIN;

    let af = [
        F_VAR,
        F_VAR,
    ];

    println!("{}", af[0](4, 0)); // 7
    println!("{}", af[1](10, 1, 2, 3)); // 13
}

fn use_vla(n: i32, p: &[i32]) -> i32 {
    (p[0] * 10 + p[n - 1]) as i32
}