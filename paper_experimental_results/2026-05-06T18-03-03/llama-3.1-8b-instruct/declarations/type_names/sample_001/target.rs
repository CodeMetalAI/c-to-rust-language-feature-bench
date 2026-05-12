fn main() {
    let g_arr = [1, 2, 3];

    let fip = || g_arr.get(1).unwrap();

    let f_plain = || 7;

    let f_var = |x: u32, _args: ()| -> i32 { x as i32 + 3 };

    let use_vla = |n: usize, p: &[i32]| -> i32 {
        p.get(0).unwrap() * 10 + p.get(n - 1).unwrap()
    };

    {
        if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
            panic!("sizeof mismatch");
        }
    }

    {
        let p = &g_arr[0] as *const i32;
        if unsafe { *p as i32 } != 1 {
            panic!("pointer cast mismatch");
        }
    }

    {
        let q = [&g_arr[0], &g_arr[1], &g_arr[2]][2];
        if *q != 3 {
            panic!("array indexing mismatch");
        }
    }

    {
        let pa = &g_arr as *const [i32; 3];
        if unsafe { (*pa).get(1).unwrap() } != 2 {
            panic!("pointer to array indexing mismatch");
        }
    }

    {
        let n = 3;
        let r = use_vla(n, &g_arr);
        if r != 13 {
            panic!("vla usage mismatch");
        }
    }

    {
        let fip = fip;
        if fip().unwrap() != 2 {
            panic!("function pointer invocation mismatch");
        }
    }

    {
        let f_plain = f_plain;
        if f_plain() != 7 {
            panic!("function pointer invocation mismatch");
        }
    }

    {
        let af = [f_var, f_var];
        if af[0](4, ()) != 7 {
            panic!("function pointer invocation mismatch");
        }

        if af[1](10, 1, 2, 3) != 13 {
            panic!("function pointer invocation mismatch");
        }
    }

    println!("All tests passed");
}