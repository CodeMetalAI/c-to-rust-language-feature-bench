fn main() {
    let expr = 0;
    let exit_code = match expr {
        0 => {
            println!("0");
            0
        }
        1 => {
            println!("1");
            1
        }
        2 => {
            println!("2");
            2,
        }
        3 => {
            println!("三");
            3,
        }
        4 => {
            println!("四");
            4,
        }
        17 => {
            println!("17");
            17,
        }
        _ => {
            println!("其他");
            0,
        }
    };

    std::process::exit(exit_code);
}