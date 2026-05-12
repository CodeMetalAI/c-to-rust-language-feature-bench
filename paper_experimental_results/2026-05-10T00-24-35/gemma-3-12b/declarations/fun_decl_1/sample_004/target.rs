static mut GSTORE: [i32; 4] = [0; 4];

fn f() -> i32 {
    unsafe {
        GSTORE[0] = 111;
        GSTORE[0] + 1
    }
}

fn fip() -> &'static mut i32 {
    unsafe {
        GSTORE[1] = 222;
        &GSTORE[1]
    }
}

static alt0: fn() -> i32 = || {
    unsafe {
        GSTORE[2] = 333;
        GSTORE[2] - 1
    }
};

static alt1: fn() -> i32 = || {
    unsafe {
        GSTORE[3] = 444;
        GSTORE[3] + 2
    }
};

static fn use_call_through(pf: fn() -> i32) -> i32 {
    pf()
}

static fn choose(x: i32) -> i32 {
    if x & 1 == 1 {
        1
    } else {
        0
    }
}

fn main() -> i32 {
    let mut pfi: fn() -> i32;

    let r_f = f();
    if r_f != 112 {
        return 1;
    }

    let v_fip = unsafe { *fip() };
    if v_fip != 222 {
        return 2;
    }

    pfi = alt0;
    if choose(unsafe { GSTORE[0] }) == 0 {
        pfi = alt1;
    }

    {
        let r_pfi = pfi();
        let r_use = use_call_through(pfi);

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