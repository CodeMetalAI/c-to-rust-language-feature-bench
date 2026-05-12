fn main() {
    let x = 17i32;
    let p = &x;
    let pc = p;
    let pv = p;
    let pcv = p;

    // Pointer equality checks are omitted as they always hold in this context,
    // preserving the behavior since the program only returns 0 in this case.

    if *pc != 17 {
        std::process::exit(4);
    }
    if *pv != 17 {
        std::process::exit(5);
    }
    if *pcv != 17 {
        std::process::exit(6);
    }

    std::process::exit(0);
}