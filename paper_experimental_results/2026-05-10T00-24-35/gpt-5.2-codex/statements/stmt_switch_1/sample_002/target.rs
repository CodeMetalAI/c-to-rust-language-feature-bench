fn main() {
    let expr = 0;

    {
        let mut i: i32 = 0;
        match expr {
            0 => {
                i = 17;
                if i == 17 {
                    return;
                } else {
                    std::process::exit(1);
                }
            }
            _ => {}
        }
    }
}