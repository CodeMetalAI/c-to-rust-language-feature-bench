fn main() {
    let g_arr: [i32; 3] = [1, 2, 3];

    fn fip() -> *const i32 { &g_arr[1] as *const i32 }

    fn f_plain() -> i32 { 7 }

    fn f_var(x: u32, _args: ()) -> i32 { (x + 3) as i32 }

    fn use_vla(n: usize, p: &[i32; n]) -> i32 { p[0] * 10 + p[n - 1] }

    if std::mem::size_of_val(&(0 as i32))!= std::mem::size_of::<i32>() {
        return;
    }

    let p = &g_arr[0] as *const i32;
    if *p!= 1 {
        return;
    }

    let q = [&g_arr[0], &g_arr[1], &g_arr[2]][2] as *const i32;
    if *q!= 3 {
        return;
    }

    let pa = &g_arr as *const [i32; 3];
    if (*pa)[1]!= 2 {
        return;
    }

    let n = 3;
    let r = use_vla(n, &g_arr);
    if r!= 13 {
        return;
    }

    let pf: fn() -> *const i32 = fip;
    if *pf()!= 2 {
        return;
    }

    let pf: fn() -> i32 = f_plain;
    if pf()!= 7 {
        return;
    }

    let af = [f_var, f_var];
    if af[0](4, ())!= 7 {
        return;
    }
    if af[1](10, ())!= 13 {
        return;
    }
}