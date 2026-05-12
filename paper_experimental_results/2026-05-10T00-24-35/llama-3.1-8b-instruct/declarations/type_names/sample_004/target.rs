fn main() {
    let mut g_arr = [1, 2, 3];

    fn fip() -> &i32 {
        &g_arr[1]
    }

    fn f_plain() -> i32 {
        7
    }

    fn f_var(x: u32, _args: ...) -> i32 {
        (x + 3u32) as i32
    }

    fn use_vla(n: i32, p: &[i32; n as usize]) -> i32 {
        (p[0] * 10 + p[(n - 1) as usize]) as i32
    }

    if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
        return 1;
    }

    {
        let p = &mut *(&mut g_arr);
        if *p != 1 {
            return 2;
        }
    }

    {
        let q = &g_arr[2];
        if *q != 3 {
            return 3;
        }
    }

    {
        let pa = &g_arr[..];
        if pa[1] != 2 {
            return 4;
        }
    }

    {
        let n = 3usize;
        let r = use_vla(n, &g_arr[0..n]);
        if r != 13 {
            return 5;
        }
    }

    {
        let pf = fip;
        if *pf() != 2 {
            return 6;
        }
    }

    {
        let pf = f_plain;
        if pf() != 7 {
            return 7;
        }
    }

    {
        let af: [&fn(u32, ...); 2] = [f_var, f_var];

        if af[0](4, 0) != 7 {
            return 8;
        }

        if af[1](10, 1, 2, 3) != 13 {
            return 9;
        }
    }

    0
}