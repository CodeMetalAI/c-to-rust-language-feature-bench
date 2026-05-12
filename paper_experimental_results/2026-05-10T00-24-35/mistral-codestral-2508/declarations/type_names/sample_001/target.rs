fn fip() -> &'static mut i32 {
    static mut G_ARR: [i32; 3] = [1, 2, 3];
    unsafe { &mut G_ARR[1] }
}

fn f_plain() -> i32 {
    7
}

fn f_var(x: u32, _: ...) -> i32 {
    x as i32 + 3
}

fn use_vla<T, const N: usize>(n: usize, p: &[T; N]) -> i32 {
    if n != N {
        panic!("Size mismatch");
    }
    (p[0].clone() as i32) * 10 + (p[n - 1].clone() as i32)
}

fn main() {
    {
        assert_eq!(std::mem::size_of::<i32>(), std::mem::size_of::<i32>());
    }

    {
        let g_arr = [1, 2, 3];
        let p = &g_arr[0];
        assert_eq!(*p, 1);
    }

    {
        let g_arr = [1, 2, 3];
        let q = &g_arr[2];
        assert_eq!(*q, 3);
    }

    {
        let g_arr = [1, 2, 3];
        let pa = &g_arr;
        assert_eq!(pa[1], 2);
    }

    {
        let g_arr = [1, 2, 3];
        let n = 3;
        let r = use_vla(n, &g_arr);
        assert_eq!(r, 13);
    }

    {
        let pf = fip;
        assert_eq!(*pf(), 2);
    }

    {
        let pf = f_plain;
        assert_eq!(pf(), 7);
    }

    {
        let af: [fn(u32, ...) -> i32; 2] = [f_var, f_var];

        assert_eq!(af[0](4, 0), 7);

        assert_eq!(af[1](10, 1, 2, 3), 13);
    }
}