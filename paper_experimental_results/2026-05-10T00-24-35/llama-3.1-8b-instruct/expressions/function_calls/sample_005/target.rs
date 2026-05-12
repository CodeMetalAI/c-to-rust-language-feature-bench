// Function calls demonstration

fn log_call(id: i32) {
    static mut LOGV: [i32; 5] = [0; 5];
    static mut LOGN: usize = 0;
    unsafe {
        if LOGN < 5 {
            LOGV[LOGN] = id;
            LOGN += 1;
        }
    }
}

fn f1() -> i32 {
    log_call(1);
    0
}

fn f2() -> i32 {
    log_call(2);
    20
}

fn f3() -> i32 {
    log_call(3);
    30
}

fn f4() -> i32 {
    log_call(4);
    40
}

fn target(a: i32, b: i32) -> i32 {
    log_call(9);
    if !f1() || !f2() || !f3() || !f4() {
        return 100;
    }
    0
}

fn main() {
    let counts = [0; 10];

    let rc = target(f1(), f3() + f4()) as usize;
    if rc != 0 {
        return;
    }

    if LOGN != 5 {
        eprintln!("LOGN != 5");
        return;
    }
    if LOGV[4] != 9 {
        eprintln!("LOGV[4] != 9");
        return;
    }

    for i in 0..4 {
        counts[LOGV[i]] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        eprintln!("counts mismatch");
        return;
    }

    Ok(())
}