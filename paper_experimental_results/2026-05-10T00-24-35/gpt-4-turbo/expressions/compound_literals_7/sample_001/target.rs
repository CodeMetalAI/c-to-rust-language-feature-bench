struct IntList<'a> {
    car: i32,
    cdr: &'a IntList<'a>,
}

fn eval(list: &IntList) -> i32 {
    if list.car != 0 {
        1
    } else if !std::ptr::eq(list.cdr, list) {
        2
    } else if !std::ptr::eq(list.cdr.cdr, list) {
        3
    } else {
        0
    }
}

fn main() {
    static mut ENDLESS_ZEROS: Option<IntList> = None;
    unsafe {
        ENDLESS_ZEROS = Some(IntList { car: 0, cdr: ENDLESS_ZEROS.as_ref().unwrap() });
        std::process::exit(eval(ENDLESS_ZEROS.as_ref().unwrap()));
    }
}