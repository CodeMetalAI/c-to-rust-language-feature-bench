fn main() {
    let mut expr = 0;
    let mut i;

    match expr {
        0 => {
            i = 4;
            i = 17;
            if i!= 17 {
                std::process::exit(1);
            }
        }
        _ => unreachable!(),
    }
}