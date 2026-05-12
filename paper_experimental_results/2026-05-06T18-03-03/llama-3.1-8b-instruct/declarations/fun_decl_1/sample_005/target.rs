// fun_decl_1.rs
static mut GSTORE: [i32; 4] = [0; 4];

fn f() -> i32 {
    GSSTORE[0] = 111;
    GSSTORE[0] + 1
}

fn fip() -> *const i32 {
    GSSTORE[1] = 222;
    &GSSTORE[1]
}

static alt0: fn() -> i32 = || {
    GSSTORE[2] = 333;
    GSSTORE[2] - 1
};

static alt1: fn() -> i32 = || {
    GSSTORE[3] = 444;
    GSSTORE[3] + 2
};

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

fn main() {
    let mut pfi: fn() -> i32 = alt0;

    let r_f = f();
    if r_f!= 112 {
        return 1;
    }

    let v_fip = unsafe { *fip() };
    if v_fip!= 222 {
        return 2;
    }

    if choose(GSSTORE[0]) == 0 {
        pfi = alt1;
    }

    let r_pfi = pfi();
    let r_use = use_call_through(pfi);

    if r_pfi!= r_use {
        return 3;
    }

    if pfi == alt0 {
        if r_pfi!= 332 {
            return 4;
        }
    } else {
        if r_pfi!= 446 {
            return 5;
        }
    }

    let q = pfi();
    if q == 0 {
        return 6;
    }

    0
}