use std::cell::RefCell;

thread_local! {
    static ADDR_COUNTER: RefCell<u64> = RefCell::new(0);
}

fn get_addr() -> u64 {
    ADDR_COUNTER.with(|counter| {
        let mut c = counter.borrow_mut();
        *c += 1;
        *c
    })
}

fn distinct_instances(depth: i32, prev_addr: Option<u64>) -> i32 {
    let addr = get_addr();

    if prev_addr.is_some() && addr == prev_addr.unwrap() {
        return 1;
    }

    if depth == 0 {
        return 0;
    }

    distinct_instances(depth - 1, Some(addr))
}

fn main() {
    if distinct_instances(8, None) != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}