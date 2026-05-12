fn main() {
    static mut GSTORE: [i32; 4] = [0; 4];

    fn f() -> i32 {
        unsafe {
            GSTORE[0] = 111;
            GSTORE[0] + 1
        }
    }

    fn fip() -> *mut i32 {
        unsafe {
            GSTORE[1] = 222;
            &mut GSTORE[1] as *mut _
        }
    }

    fn alt0() -> i32 {
        unsafe {
            GSTORE[2] = 333;
            GSTORE[2] - 1
        }
    }

    fn alt1() -> i32 {
        unsafe {
            GSTORE[3] = 444;
            GSTORE[3] + 2
        }
    }

    fn use_call_through(pf: fn() -> i32) -> i32 {
        pf()
    }

    fn choose(x: i32) -> i32 {
        if x & 1!= 0 {
            1
        } else {
            0
        }
    }

    let r_f = f();
    if r_f!= 112 {
        std::process::exit(1);
    }

    let v_fip = unsafe { *fip() };
    if v_fip!= 222 {
        std::process::exit(2);
    }

    let mut pfi: fn() -> i32 = alt0;
    if choose(unsafe { GSTORE[0] }) == 0 {
        pfi = alt1;
    }

    {
        let r_pfi = pfi();
        let r_use = use_call_through(pfi);

        if r_pfi!= r_use {
            std::process::exit(3);
        }

        if pfi as usize == alt0 as usize {
            if r_pfi!= 332 {
                std::process::exit(4);
            }
        } else {
            if r_pfi!= 446 {
                std::process::exit(5);
            }
        }
    }

    {
        let q = pfi();
        if q == 0 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}