fn main() {
    let mut acc = 0;
    let mut first_time = true;
    let mut i = 0;
    
    loop {
        if !first_time {
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
        }
        
        // first_time label
        acc = 15;
        first_time = false;
        
        // continue goes here
        i += 1;
        if i > 5 {
            break;
        }
    }
    
    std::process::exit(if acc == 20 { 0 } else { 1 });
}