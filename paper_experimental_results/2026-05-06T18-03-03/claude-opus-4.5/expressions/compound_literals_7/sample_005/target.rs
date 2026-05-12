struct IntList {
    car: i32,
    cdr_index: usize,
}

static ENDLESS_ZEROS: [IntList; 1] = [IntList { car: 0, cdr_index: 0 }];

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    // Check if cdr points to endless_zeros (index 0)
    if x.cdr_index != 0 {
        return 2;
    }
    // Check if cdr->cdr points to endless_zeros
    let cdr = &ENDLESS_ZEROS[x.cdr_index];
    if cdr.cdr_index != 0 {
        return 3;
    }
    0
}

fn main() {
    std::process::exit(eval(&ENDLESS_ZEROS[0]));
}