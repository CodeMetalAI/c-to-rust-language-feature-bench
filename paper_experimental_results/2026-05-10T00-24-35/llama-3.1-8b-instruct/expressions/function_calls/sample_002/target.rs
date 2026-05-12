#[derive(Debug)]
struct LogEntry {
    id: i32,
}

static mut SEEN_F1: i32 = 0;
static mut SEEN_F2: i32 = 0;
static mut SEEN_F3: i32 = 0;
static mut SEEN_F4: i32 = 0;
static mut LOGV: [Option<LogEntry>; 5] = [None; 5];
static mut LOGN: usize = 0;

fn log_call(id: i32) {
    if LOGN < 5 {
        LOGV[LOGN] = Some(LogEntry { id });
        LOGN += 1;
    }
}

fn f1() -> i32 {
    unsafe {
        SEEN_F1 = 1;
    }
    log_call(1);
    0
}

fn f2() -> i32 {
    unsafe {
        SEEN_F2 = 1;
    }
    log_call(2);
    20
}

fn f3() -> i32 {
    unsafe {
        SEEN_F3 = 1;
    }
    log_call(3);
    30
}

fn f4() -> i32 {
    unsafe {
        SEEN_F4 = 1;
    }
    log_call(4);
    40
}

type Fn2 = fn(i32, i32) -> i32;

fn target(_: i32, _: i32) -> i32 {
    log_call(9);
    if !(unsafe { SEEN_F1 == 1 } && unsafe { SEEN_F2 == 1 } && unsafe { SEEN_F3 == 1 } && unsafe { SEEN_F4 == 1 }) {
        100
    } else {
        0
    }
}

fn main() {
    let mut counts: [usize; 10] = [0; 10];

    fn2 pf = |a: i32, b: i32| -> i32 {
        target(a, b)
    };

    let result = pf(f1(), (f3() + f4()) as i32);

    if result != 0 {
        return;
    }

    if LOGN != 5 {
        panic!("Expected LOGN to be 5");
    }

    if LOGV[4].map_or(false, |e| e.id != 9) {
        panic!("Expected LOGV[4] to be 9");
    }

    for (i, entry) in LOGV.iter().enumerate() {
        if let Some(entry) = entry {
            counts[entry.id as usize] += 1;
        }
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        panic!("Expected counts to be 1 for each entry");
    }

    assert_eq!(result, 0);
}