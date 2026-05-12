use std::cell::Cell;
use std::ptr;
use std::sync::OnceLock;

struct IntList {
    car: i32,
    cdr: Cell<Option<&'static IntList>>,
}

static ENDLESS: OnceLock<&'static IntList> = OnceLock::new();

fn endless_zeros() -> &'static IntList {
    ENDLESS.get_or_init(|| {
        let list = Box::leak(Box::new(IntList {
            car: 0,
            cdr: Cell::new(None),
        }));
        list.cdr.set(Some(list));
        list
    })
}

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    let cdr = match x.cdr.get() {
        Some(c) => c,
        None => return 2,
    };
    if !ptr::eq(cdr, endless_zeros()) {
        return 2;
    }
    let cdr_cdr = match cdr.cdr.get() {
        Some(c) => c,
        None => return 3,
    };
    if !ptr::eq(cdr_cdr, endless_zeros()) {
        return 3;
    }
    0
}

fn main() {
    let result = eval(endless_zeros());
    std::process::exit(result);
}