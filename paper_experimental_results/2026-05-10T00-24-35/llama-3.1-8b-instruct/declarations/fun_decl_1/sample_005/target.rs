fn f() -> i32 {
    let mut gstore = [0; 4];
    gstore[0] = 111;
    gstore[0] + 1
}

fn fip() -> *mut i32 {
    let mut gstore = [0; 4];
    gstore[1] = 222;
    &gstore[1]
}

fn alt0() -> i32 {
    let mut gstore = [0; 4];
    gstore[2] = 333;
    gstore[2] - 1
}

fn alt1() -> i32 {
    let mut gstore = [0; 4];
    gstore[3] = 444;
    gstore[3] + 2
}

fn use_call_through(pf: fn() -> i32) -> i32 {
    pf()
}

fn choose(x: i32) -> i32 {
    if x & 1 != 0 {
        1
    } else {
        0
    }
}

fn main() {
    let r_f = f();
    if r_f != 112 {
        panic!("test failed");
    }

    let v_fip = unsafe { *fip() };
    if v_fip != 222 {
        panic!("test failed");
    }

    let mut pfi: fn() -> i32 = alt0;
    if choose(f() & 1) == 0 {
        pfi = alt1;
    }

    let r_pfi = pfi();
    let r_use = use_call_through(pfi);

    if r_pfi != r_use {
        panic!("test failed");
    }

    if pfi == alt0 {
        if r_pfi != 332 {
            panic!("test failed");
        }
    } else {
        if r_pfi != 446 {
            panic!("test failed");
        }
    }

    let q = pfi();
    if q == 0 {
        panic!("test failed");
    }
}