fn main() {
    let mut acc: i32 = 0;
    let mut i: i32 = 0;
    let mut first_time = true;

    loop {
        if !first_time {
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
        }
        
        // first_time label equivalent
        acc = 15;
        first_time = false;
        
        // continue equivalent - go to increment
        i += 1;
        if i > 5 {
            break;
        }
    }

    std::process::exit(if acc == 20 { 0 } else { 1 });
}