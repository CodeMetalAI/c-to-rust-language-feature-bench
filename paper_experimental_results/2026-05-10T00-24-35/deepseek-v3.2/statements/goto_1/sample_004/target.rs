fn main() {
    let mut acc = 0;
    let mut first_time = true;
    let mut i = 0;
    
    while i < 5 {
        if !first_time {
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
        }
        
        first_time = false;
        acc = 15;
        
        i += 1;
        continue;
    }
    
    std::process::exit(if acc == 20 { 0 } else { 1 });
}