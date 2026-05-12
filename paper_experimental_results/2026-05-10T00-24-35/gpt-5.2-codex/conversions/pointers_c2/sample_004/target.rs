use std::process::exit;

fn main() {
    let x = 17i32;
    let p: &i32 = &x;

    let pc: &i32 = p;
    let pv: &i32 = p;
    let pcv: &i32 = p;

    if !std::ptr::eq(pc, p) {
        exit(1);
    }
    if !std::ptr::eq(pv, p) {
        exit(2);
    }
    if !std::ptr::eq(pcv, p) {
        exit(3);
    }

    if *pc != 17 {
        exit(4);
    }
    if *pv != 17 {
        exit(5);
    }
    if *pcv != 17 {
        exit(6);
    }

    exit(0);
}