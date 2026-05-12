fn main() {
    let g_arr = [1, 2, 3];

    let fip = || g_arr[1];

    let f_plain = || 7;

    let f_var = |x: u32, ...| -> i32 { x as i32 + 3 };

    let use_vla = |n: usize, p: &[i32]| -> i32 {
        p[0] * 10 + p[n - 1]
    };

    {
        if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
            return 1;
        }
    }

    {
        let p = &g_arr[0];
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
        let pa = &g_arr as *const [i32; 3];
        if pa[1] != 2 {
            return 4;
        }
    }

    {
        let n = 3;
        let r = use_vla(n, &g_arr);
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
        let af = [f_var, f_var];

        if af[0](4, 0) != 7 {
            return 8;
        }

        if af[1](10, 1, 2, 3) != 13 {
            return 9;
        }
    }

    0
}