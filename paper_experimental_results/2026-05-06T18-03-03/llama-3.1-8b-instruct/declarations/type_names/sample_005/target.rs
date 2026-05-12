fn main() {
    let g_arr = [1, 2, 3];

    let fip = || g_arr.get(1).unwrap();

    let f_plain = || 7;

    let f_var = |x: u32, _args: ()| -> i32 { (x + 3) as i32 };

    let use_vla = |n: usize, p: &[i32]| -> i32 {
        p.get(0).unwrap() * 10 + p.get(n - 1).unwrap()
    };

    {
        if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
            std::process::exit(1);
        }
    }

    {
        let p = &g_arr[0] as *const i32;
        if unsafe { *p as i32 } != 1 {
            std::process::exit(2);
        }
    }

    {
        let q = [&g_arr[0], &g_arr[1], &g_arr[2]][2];
        if *q != 3 {
            std::process::exit(3);
        }
    }

    {
        let pa = &g_arr as *const [i32; 3];
        if pa[1] != 2 {
            std::process::exit(4);
        }
    }

    {
        let n = 3;
        let r = use_vla(n, &g_arr);
        if r != 13 {
            std::process::exit(5);
        }
    }

    {
        let pf = || fip();
        if *pf() != 2 {
            std::process::exit(6);
        }
    }

    {
        let pf = f_plain;
        if pf() != 7 {
            std::process::exit(7);
        }
    }

    {
        let af = [f_var, f_var];

        if af[0](4, ()) != 7 {
            std::process::exit(8);
        }

        if af[1](10, 1, 2, 3) != 13 {
            std::process::exit(9);
        }
    }

    std::process::exit(0);
}