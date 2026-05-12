fn main() {
    static mut gstore: [i32; 4] = [0; 4];

    fn f() -> i32 {
        unsafe {
            gstore[0] = 111;
            gstore[0] + 1
        }
    }

    fn fip() -> &'static mut i32 {
        unsafe {
            gstore[1] = 222;
            &gstore[1]
        }
    }

    fn alt0() -> i32 {
        unsafe {
            gstore[2] = 333;
            gstore[2] - 1
        }
    }

    fn alt1() -> i32 {
        unsafe {
            gstore[3] = 444;
            gstore[3] + 2
        }
    }

    fn use_call_through(pf: &dyn Fn() -> i32) -> i32 {
        let r = pf();
        r
    }

    fn choose(x: i32) -> i32 {
        if x & 1 == 1 {
            1
        } else {
            0
        }
    }

    let r_f = f();
    if r_f != 112 {
        return 1;
    }

    let v_fip = unsafe { *fip() };
    if v_fip != 222 {
        return 2;
    }

    let mut pfi: fn() -> i32;
    pfi = alt0;

    if choose(unsafe { gstore[0] }) == 0 {
        pfi = alt1;
    }

    {
        let r_pfi = pfi();
        let r_use = use_call_through(&pfi);

        if r_pfi != r_use {
            return 3;
        }

        if pfi == alt0 {
            if r_pfi != 332 {
                return 4;
            }
        } else {
            if r_pfi != 446 {
                return 5;
            }
        }
    }

    {
        let q = pfi();
        if q == 0 {
            return 6;
        }
    }

    0
}