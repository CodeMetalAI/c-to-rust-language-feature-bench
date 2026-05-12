fn main() {
    let x: i32 = 17;
    let p: *const i32 = &x;
    let pc: *const i32 = p;
    let pv: *mut i32 = p;
    let pcv: *const i32 = p;

    if pc != p {
        println!("Error: pc != p");
        std::process::exit(1);
    }
    if pv != p {
        println!("Error: pv != p");
        std::process::exit(2);
    }
    if pcv != p {
        println!("Error: pcv != p");
        std::process::exit(3);
    }

    if *pc != 17 {
        println!("Error: *pc != 17");
        std::process::exit(4);
    }
    if *pv != 17 {
        println!("Error: *pv != 17");
        std::process::exit(5);
    }
    if *pcv != 17 {
        println!("Error: *pcv != 17");
        std::process::exit(6);
    }

    println!("All checks passed");
    std::process::exit(0);
}