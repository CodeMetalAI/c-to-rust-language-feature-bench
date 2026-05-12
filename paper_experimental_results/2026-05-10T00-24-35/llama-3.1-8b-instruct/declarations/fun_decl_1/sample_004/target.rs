// fun_decl_1.rs
static mut gstore: [i32; 4] = [0; 4];

fn f() -> i32 {
    gstore[0] = 111;
    gstore[0] + 1
}

fn fip() -> *const i32 {
    gstore[1] = 222;
    &gstore[1]
}

static alt0: fn() -> i32 = || {
    gstore[2] = 333;
    gstore[2] - 1
};

static alt1: fn() -> i32 = || {
    gstore[3] = 444;
    gstore[3] + 2
};

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
    let pfi: fn() -> i32 = alt0;

    let r_f = f();
    if r_f != 112 {
        std::process::exit(1);
    }

    let v_fip = *fip();
    if v_fip != 222 {
        std::process::exit(2);
    }

    if choose(gstore[0]) == 0 {
        pfi = alt1
    }

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

    let q = pfi();
    if q == 0 {
        std::process::exit(6);
    }
}