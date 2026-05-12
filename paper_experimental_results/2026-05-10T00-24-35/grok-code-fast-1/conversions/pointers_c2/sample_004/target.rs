use std::process;

fn main() {
    let mut x = 17;
    let p = &mut x;
    let pc = p as &i32;
    let pv = p;
    let pcv = p as &i32;

    if *pc != 17 {
        process::exit(4);
    }
    if *pv != 17 {
        process::exit(5);
    }
    if *pcv != 17 {
        process::exit(6);
    }

    process::exit(0);
}