fn main() {
    let expr = 0;
    let exit_code = match expr {
        0 => {
            let mut i = 0;
            i = 17;
            if i == 17 { 0 } else { 1 }
        }
        _ => 0,
    };
    std::process::exit(exit_code);
}