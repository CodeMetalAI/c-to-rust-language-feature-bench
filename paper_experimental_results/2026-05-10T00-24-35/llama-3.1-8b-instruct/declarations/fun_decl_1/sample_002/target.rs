// fun_decl_1.rs
static mut GSTORE: [i32; 4] = [0; 4];

fn f() -> i32 {
    unsafe {
        GSSTORE[0] = 111;
        GSSTORE[0] + 1
    }
}

fn fip() -> *const i32 {
    unsafe {
        GSSTORE[1] = 222;
        &GSSTORE[1]
    }
}

fn alt0() -> i32 {
    unsafe {
        GSSTORE[2] = 333;
        GSSTORE[2] - 1
    }
}

fn alt1() -> i32 {
    unsafe {
        GSSTORE[3] = 444;
        GSSTORE[3] + 2
    }
}

fn use_call_through(pf: fn() -> i32) -> i32 {
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
    let mut pfi: fn() -> i32 = alt0;

    let r_f = f();
    if r_f!= 112 {
        println!("Test failed: f()!= 112");
        std::process::exit(1);
    }

    let v_fip = *fip();
    if v_fip!= 222 {
        println!("Test failed: *fip()!= 222");
        std::process::exit(2);
    }

    if choose(GSTORE[0]) == 0 {
        pfi = alt1;
    }

    let r_pfi = pfi();
    let r_use = use_call_through(pfi);

    if r_pfi!= r_use {
        println!("Test failed: pfi()!= use_call_through(pfi())");
        std::process::exit(3);
    }

    if pfi == alt0 {
        if r_pfi!= 332 {
            println!("Test failed: alt0()!= 332");
            std::process::exit(4);
        }
    } else {
        if r_pfi!= 446 {
            println!("Test failed: alt1()!= 446");
            std::process::exit(5);
        }
    }

    let q = pfi();
    if q == 0 {
        println!("Test failed: pfi()!= 0");
        std::process::exit(6);
    }

    println!("Test passed!");
}