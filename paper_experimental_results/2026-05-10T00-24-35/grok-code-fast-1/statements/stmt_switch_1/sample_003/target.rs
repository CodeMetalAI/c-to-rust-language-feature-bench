fn main() {
    let expr = 0;
    match expr {
        0 => {
            let mut i = 4;
            i = 17;
            std::process::exit(if i == 17 { 0 } else { 1 });
        }
        _ => {
            // fall through, do nothing
        }
    }
    // implicit return, exit code 0
}