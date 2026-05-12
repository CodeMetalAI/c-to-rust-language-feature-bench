struct IntList {
    car: i32,
    cdr_index: usize,
}

static ENDLESS_ZEROS: IntList = IntList { car: 0, cdr_index: 0 };

fn get_list(index: usize) -> &'static IntList {
    match index {
        0 => &ENDLESS_ZEROS,
        _ => &ENDLESS_ZEROS,
    }
}

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    let cdr = get_list(x.cdr_index);
    if !std::ptr::eq(cdr, &ENDLESS_ZEROS) {
        return 2;
    }
    let cdr_cdr = get_list(cdr.cdr_index);
    if !std::ptr::eq(cdr_cdr, &ENDLESS_ZEROS) {
        return 3;
    }
    0
}

fn main() {
    std::process::exit(eval(&ENDLESS_ZEROS));
}