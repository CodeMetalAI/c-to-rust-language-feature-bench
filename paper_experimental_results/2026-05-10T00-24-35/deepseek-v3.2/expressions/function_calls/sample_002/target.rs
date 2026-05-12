fn main() {
    use std::cell::Cell;

    thread_local! {
        static SEEN_F1: Cell<bool> = Cell::new(false);
        static SEEN_F2: Cell<bool> = Cell::new(false);
        static SEEN_F3: Cell<bool> = Cell::new(false);
        static SEEN_F4: Cell<bool> = Cell::new(false);
        static LOGV: Cell<[i32; 5]> = Cell::new([0; 5]);
        static LOGN: Cell<usize> = Cell::new(0);
    }

    fn log_call(id: i32) {
        LOGN.with(|logn| {
            let n = logn.get();
            if n < 5 {
                LOGV.with(|logv| {
                    let mut arr = logv.get();
                    arr[n] = id;
                    logv.set(arr);
                });
                logn.set(n + 1);
            }
        });
    }

    fn f1() -> i32 {
        SEEN_F1.with(|seen| seen.set(true));
        log_call(1);
        0
    }

    fn f2() -> i32 {
        SEEN_F2.with(|seen| seen.set(true));
        log_call(2);
        20
    }

    fn f3() -> i32 {
        SEEN_F3.with(|seen| seen.set(true));
        log_call(3);
        30
    }

    fn f4() -> i32 {
        SEEN_F4.with(|seen| seen.set(true));
        log_call(4);
        40
    }

    type Fn2 = fn(i32, i32) -> i32;

    fn target(a: i32, b: i32) -> i32 {
        let _ = a;
        let _ = b;
        log_call(9);

        let ok = SEEN_F1.with(|seen| seen.get())
            && SEEN_F2.with(|seen| seen.get())
            && SEEN_F3.with(|seen| seen.get())
            && SEEN_F4.with(|seen| seen.get());
        if !ok {
            return 100;
        }

        0
    }

    let pf: [Fn2; 1] = [target];

    let rc = pf[f1() as usize](f2(), f3() + f4());
    if rc != 0 {
        std::process::exit(rc);
    }

    let logn = LOGN.with(|l| l.get());
    if logn != 5 {
        std::process::exit(1);
    }

    let logv = LOGV.with(|l| l.get());
    if logv[4] != 9 {
        std::process::exit(2);
    }

    let mut counts = [0; 10];
    for i in 0..4 {
        counts[logv[i] as usize] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        std::process::exit(3);
    }

    std::process::exit(0);
}