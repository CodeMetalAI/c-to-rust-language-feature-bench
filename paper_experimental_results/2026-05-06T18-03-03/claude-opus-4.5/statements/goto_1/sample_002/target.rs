fn main() {
    let mut acc: i32 = 0;
    let mut i: i32 = 0;
    let mut first_entry = true;

    loop {
        if !first_entry {
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
        }
        
        // first_time label
        acc = 15;
        first_entry = false;
        
        // continue equivalent - go to increment
        i += 1;
        if i < 5 {
            continue;
        } else {
            break;
        }
    }

    std::process::exit(if acc == 20 { 0 } else { 1 });
}