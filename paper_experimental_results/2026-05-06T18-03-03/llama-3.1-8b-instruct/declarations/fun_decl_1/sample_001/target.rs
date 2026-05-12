static mut gstore: [i32; 4] = [0; 4];

fn f() -> i32 {
    gstore[0] = 111;
    gstore[0] + 1
}

fn fip() -> *const i32 {
    gstore[1] = 222;
    &gstore[1]
}

fn alt0() -> i32 {
    gstore[2] = 333;
    gstore[2] - 1
}

fn alt1() -> i32 {
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
    let pfi: fn() -> i32;

    let r_f = f();
    if r_f != 112 {
        return 1;
    }

    let v_fip = *fip();
    if v_fip != 222 {
        return 2;
    }

    pfi = alt0;
    if choose(gstore[0]) == 0 {
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

    return 0;
}