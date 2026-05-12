static mut GSTORE: [i32; 4] = [0; 0, 0, 0];

fn f() -> i32 {
    unsafe {
        GSTORE[0] = 111;
        GSTORE[0]
    }
}

fn fip() -> *mut i32 {
    unsafe {
        GSTORE[1] = 222;
        &mut GSTORE[1]
    }
}

fn alt0() -> i32 {
    unsafe {
        GSTORE[2] = 333;
        GSTORE[2]
    }
}

fn alt 1() -> i32 {
    unsafe {
        GSTORE[3] = 444;
        GSTORE[3]
    }
}

fn use_call_through(pf: fn() -> i32) -> i32 {
    pf()
}

fn choose(x: i32) -> fn() -> i32 {
    if x & 1 != 0 {
        || f
    } else {
        || 0
    }
}

fn main() -> i32 {
    let r_f = f();
    if r_f != 112 {
        return 1;
    }

    let v_fip = fip();
    if unsafe { *v_fip != 222 } {
        return 2;
    }

    let mut pfi: fn() -> i32 = alt 0;
    if choose(GSTORE[0]) == 0 {
        pfi = alt 1;
    }

    let r_pfi = pfi();
    let r_use = use_call_through(pfi);
    if r_pfi != r_use {
        return 3;
    }

    if pfi == alt 0 {
        if r_pfi != 332 {
            return 4;
        }
    } else {
        if r_pfi != 446 {
            return 5;
        }
    }

    let q = pfi();
    if q == 0 {
        return 6;
    }

    0
}