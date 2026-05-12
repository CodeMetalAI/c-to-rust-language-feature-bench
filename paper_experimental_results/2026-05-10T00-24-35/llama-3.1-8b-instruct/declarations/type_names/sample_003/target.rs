fn main() {
    let g_arr = [1, 2, 3];

    let fip = || g_arr[1..].get(0).unwrap();

    let f_plain = || 7;

    let f_var = |x: u32, _args: ()| x + 3;

    let use_vla = |n: usize, p: &[i32; n]| p[0] * 10 + p[n - 1];

    {
        if std::mem::size_of::<i32>() != 4 {
            eprintln!("sizeof((int){0}) != sizeof(int)");
            std::process::exit(1);
        }
    }

    {
        let p: *const i32 = &g_arr[0];
        if unsafe { *p as i32 != 1 } {
            eprintln!("*(int *)(p) != 1");
            std::process::exit(2);
        }
    }

    {
        let q = [&g_arr[0], &g_arr[1], &g_arr[2]][2];
        if *q != 3 {
            eprintln!("*q != 3");
            std::process::exit(3);
        }
    }

    {
        let pa: &[i32; 3] = &g_arr;
        if pa[1] != 2 {
            eprintln!("(*pa)[1] != 2");
            std::process::exit(4);
        }
    }

    {
        let n = 3;
        let r = use_vla(n, &g_arr[..n]);
        if r != 13 {
            eprintln!("r != 13");
            std::process::exit(5);
        }
    }

    {
        let fip = fip;
        if fip() != 2 {
            eprintln!("*pf() != 2");
            std::process::exit(6);
        }
    }

    {
        let f_plain = f_plain;
        if f_plain() != 7 {
            eprintln!("pf() != 7");
            std::process::exit(7);
        }
    }

    {
        let f_var = [f_var, f_var];
        if f_var[0](4, ()) != 7 {
            eprintln!("f_var[0](4) != 7");
            std::process::exit(8);
        }

        if f_var[1](10, 1, 2, 3) != 13 {
            eprintln!("f_var[1](10, 1, 2, 3) != 13");
            std::process::exit(9);
        }
    }

    std::process::exit(0);
}