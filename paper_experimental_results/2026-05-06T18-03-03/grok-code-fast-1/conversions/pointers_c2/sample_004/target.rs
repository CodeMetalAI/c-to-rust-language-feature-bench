fn main() {
    let x = 17;
    let p = &x;
    let pc = p;
    let pv = p;
    let pcv = p;

    if *pc != 17 {
        std::process::exit(4);
    }
    if *pv != 17 {
        std::process::exit(5);
    }
    if *pcv != 17 {
        std::process::exit(6);
    }
}