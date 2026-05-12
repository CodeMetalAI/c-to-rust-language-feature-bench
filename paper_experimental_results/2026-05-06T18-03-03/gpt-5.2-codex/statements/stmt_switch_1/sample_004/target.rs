fn main() {
    let expr = 0;
    let mut ret = 0;

    match expr {
        0 => {
            let mut i = 0;
            i = 17;
            ret = if i == 17 { 0 } else { 1 };
        }
        _ => {}
    }

    std::process::exit(ret);
}