fn main() {
    let mut acc = 0;
    let mut i = 0;
    let mut first_time = true;
    
    loop {
        if first_time {
            first_time = false;
        } else {
            if i >= 5 {
                break;
            }
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
        }
        
        acc = 15;
        i += 1;
    }
    
    std::process::exit(if acc == 20 { 0 } else { 1 });
}