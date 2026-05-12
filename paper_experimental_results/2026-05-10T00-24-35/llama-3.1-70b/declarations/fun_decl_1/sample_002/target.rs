static mut gstore: [i32; 4] = [0; 4];

fn f() -> i32 {
    unsafe { gstore[0] = 111; }
    unsafe { gstore[0] + 1 }
}

fn fip() -> *mut i32 {
    unsafe { gstore[1] = 222; }
    &mut gstore[1] as *mut _
}

fn alt0() -> i32 {
    unsafe { gstore[2] = 333; }
    unsafe { gstore[2] - 1 }
}

fn alt1() -> i32 {
    unsafe { gstore[3] = 444; }
    unsafe { gstore[3] + 2 }
}

type FuncPtr = fn() -> i32;

fn use_call_through(pf: FuncPtr) -> i32 {
    pf()
}

fn choose(x: i32) -> i32 {
    if x & 1 == 1 {
        1
    } else {
        0
    }
}

fn main() {
    let mut pfi: FuncPtr = alt0;

    let r_f = f();
    if r_f != 112 {
        std::process::exit(1);
    }

    let v_fip = unsafe { *fip() };
    if v_fip != 222 {
        std::process::exit(2);
    }

    if choose(unsafe { gstore[0] }) == 0 {
        pfi = alt1;
    }

    {
        let r_pfi = pfi();
        let r_use = use_call_through(pfi);

        if r_pfi != r_use {
            std::process::exit(3);
        }

        if pfi == alt0 {
            if r_pfi != 332 {
                std::process::exit(4);
            }
        } else {
            if r_pfi != 446 {
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
}