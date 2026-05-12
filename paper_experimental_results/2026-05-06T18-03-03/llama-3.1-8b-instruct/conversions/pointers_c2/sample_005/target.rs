fn main() {
    let x: i32 = 17;
    let p: *const i32 = &x;

    let pc: *const i32 = p;
    let pv: *const i32 = p;
    let pcv: *const i32 = p;

    if pc != p {
        panic!("pc != p");
    }
    if pv != p {
        panic!("pv != p");
    }
    if pcv != p {
        panic!("pcv != p");
    }

    if *pc != 17 {
        panic!("*pc != 17");
    }
    if *pv != 17 {
        panic!("*pv != 17");
    }
    if *pcv != 17 {
        panic!("*pcv != 17");
    }

    println!("All checks passed");
}