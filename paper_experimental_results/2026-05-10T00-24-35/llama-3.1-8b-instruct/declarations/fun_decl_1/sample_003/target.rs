fn f() -> i32 {
    static mut GSTORE: [i32; 4] = [0; 4];
    GSSTORE[0] = 111;
    GSSTORE[0] + 1
}

fn fip() -> *const i32 {
    static mut GSSTORE: [i32; 4] = [0; 4];
    GSSTORE[1] = 222;
    &GSSTORE[1]
}

static mut ALT0: fn() -> i32 = f;

static mut ALT1: fn() -> i32 = f;

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
    let pfi: fn() -> i32;

    let r_f = f();
    if r_f!= 112 {
        println!("Error: f() returned unexpected value");
        return;
    }

    let v_fip = *fip();
    if v_fip!= 222 {
        println!("Error: fip() returned unexpected value");
        return;
    }

    pfi = alt0;
    if choose(r_f) == 0 {
        pfi = alt1;
    }

    {
        let r_pfi = pfi();
        let r_use = use_call_through(pfi);

        if r_pfi!= r_use {
            println!("Error: use_call_through() returned unexpected value");
            return;
        }

        if std::mem::transmute(pfi) == alt0 {
            if r_pfi!= 332 {
                println!("Error: alt0() returned unexpected value");
                return;
            }
        } else {
            if r_pfi!= 446 {
                println!("Error: alt1() returned unexpected value");
                return;
            }
        }
    }

    {
        let q = pfi();
        if q!= 0 {
            println!("Error: alt0/alt1() returned unexpected value");
            return;
        }
    }
}