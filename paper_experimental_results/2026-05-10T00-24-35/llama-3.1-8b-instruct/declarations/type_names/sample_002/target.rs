fn main() {
    let g_arr = [1, 2, 3];

    let fip = || g_arr.as_slice().get(1).unwrap();

    let f_plain = || 7;

    let f_var = |x: u32, _args: ()| -> i32 { (x + 3) as i32 };

    let use_vla = |n: usize, p: &[i32]| -> i32 {
        p.get(0).unwrap() * 10 + p.get(n - 1).unwrap()
    };

    {
        if std::mem::size_of::<i32>() != std::mem::size_of::<usize>() {
            return 1;
        }
    }

    {
        let p = &g_arr[0] as *const i32;
        if unsafe { *p as i32 } != 1 {
            return 2;
        }
    }

    {
        let q = [&g_arr[0], &g_arr[1], &g_arr[2]][2];
        if *q != 3 {
            return 3;
        }
    }

    {
        let pa = &g_arr as *const [i32; 3];
        if unsafe { pa.as_ref().unwrap()[1] } != 2 {
            return 4;
        }
    }

    {
        let n = 3;
        if use_vla(n, &g_arr) != 13 {
            return 5;
        }
    }

    {
        let fip = || fip();
        if fip() != 2 {
            return 6;
        }
    }

    {
        let f_plain = f_plain;
        if (f_plain)() != 7 {
            return 7;
        }
    }

    {
        let mut af = [f_var, f_var];

        if af[0](4, ()) != 7 {
            return 8;
        }

        if af[1](10, 1, 2, 3) != 13 {
            return 9;
        }
    }

    return 0;
}