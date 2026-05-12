fn main() {
    static mut G_ARR: [i32; 3] = [1, 2, 3];

    fn fip() -> *mut i32 {
        unsafe { G_ARR.as_mut_ptr().offset(1) }
    }

    fn f_plain() -> i32 {
        7
    }

    fn f_var(x: u32) -> i32 {
        (x + 3u32) as i32
    }

    fn use_vla(n: usize, p: &[i32]) -> i32 {
        p[0] * 10 + p[n - 1]
    }

    {
        if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
            std::process::exit(1);
        }
    }

    {
        let p = unsafe { G_ARR.as_mut_ptr() };
        if unsafe { *p } != 1 {
            std::process::exit(2);
        }
    }

    {
        let q = unsafe { [G_ARR.as_mut_ptr(), G_ARR.as_mut_ptr().offset(1), G_ARR.as_mut_ptr().offset(2)][2] };
        if unsafe { *q } != 3 {
            std::process::exit(3);
        }
    }

    {
        let pa = unsafe { &G_ARR };
        if pa[1] != 2 {
            std::process::exit(4);
        }
    }

    {
        let n = 3;
        let r = use_vla(n, unsafe { &G_ARR });
        if r != 13 {
            std::process::exit(5);
        }
    }

    {
        let pf: fn() -> *mut i32 = fip;
        if unsafe { *pf() } != 2 {
            std::process::exit(6);
        }
    }

    {
        let pf: fn() -> i32 = f_plain;
        if pf() != 7 {
            std::process::exit(7);
        }
    }

    {
        let af: [fn(u32) -> i32; 2] = [f_var, f_var];

        if af[0](4u32) != 7 {
            std::process::exit(8);
        }

        if af[1](10u32) != 13 {
            std::process::exit(9);
        }
    }

    std::process::exit(0);
}