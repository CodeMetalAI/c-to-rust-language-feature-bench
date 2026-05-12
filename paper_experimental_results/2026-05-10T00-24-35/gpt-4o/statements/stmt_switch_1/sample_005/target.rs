fn main() {
    let expr = 0;
    let mut i;
    i = 4; // This initialization is technically outside the match equivalent

    match expr {
        0 => {
            i = 17;
            if i == 17 {
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        }
        _ => {}
    }
}