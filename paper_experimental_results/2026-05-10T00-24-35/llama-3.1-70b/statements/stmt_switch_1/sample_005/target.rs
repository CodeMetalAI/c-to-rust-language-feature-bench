fn main() {
    let mut expr: i32 = 0;
    let mut i: i32;

    match expr {
        _ => {
            i = 4;
            if expr == 0 {
                i = 17;
            }
            if i == 17 {
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        }
    }
}