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
        &mut GSTORE[1] as *mut i32
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
    let r = pf();
    r
}

fn choose(x: i32) -> i32 {
    if x & 1 != 0 {
        return 1;
    }
    0
}

fn main() -> std::process::ExitCode {
    let r_f = f();
    if r_f != 112 {
        return std::process::ExitCode::from(1);
    }

    let v_fip = unsafe { *fip() };
    if v_fip != 222 {
        return std::process::ExitCode::from(2);
    }

    let mut pfi: fn() -> i32 = alt0;
    if choose(unsafe { GSTORE[0] }) == 0 {
        pfi = alt1;
    }

    {
        let r_pfi = pfi();
        let r_use = use_call_through(pfi);

        if r_pfi != r_use {
            return std::process::ExitCode::from(3);
        }

        if pfi as usize == alt0 as usize {
            if r_pfi != 332 {
                return std::process::ExitCode::from(4);
            }
        } else {
            if r_pfi != 446 {
                return std::process::ExitCode::from(5);
            }
        }
    }

    {
        let q = pfi();
        if q == 0 {
            return std::process::ExitCode::from(6);
        }
    }

    std::process::ExitCode::from(0)
}