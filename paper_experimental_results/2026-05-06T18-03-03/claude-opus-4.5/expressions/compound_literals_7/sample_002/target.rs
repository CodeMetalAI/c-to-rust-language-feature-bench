struct IntList {
    car: i32,
    cdr: usize, // index into a list of IntLists
}

fn eval(list: &[IntList], idx: usize) -> i32 {
    let x = &list[idx];
    if x.car != 0 {
        return 1;
    }
    // Check if cdr points to the "endless_zeros" (index 0)
    if x.cdr != 0 {
        return 2;
    }
    // Check if cdr->cdr points to "endless_zeros" (index 0)
    if list[x.cdr].cdr != 0 {
        return 3;
    }
    0
}

fn main() {
    // endless_zeros is at index 0, with car=0 and cdr pointing to index 0 (itself)
    let endless_zeros = [IntList { car: 0, cdr: 0 }];
    std::process::exit(eval(&endless_zeros, 0));
}