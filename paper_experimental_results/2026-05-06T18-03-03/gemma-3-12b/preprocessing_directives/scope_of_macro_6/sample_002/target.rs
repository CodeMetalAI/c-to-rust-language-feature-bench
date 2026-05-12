fn main() {
    let obj_like = {
        #[cfg(feature = "NEGATIVE")]
        {
            0.0
        }
        else {
            0
        }
    };

    let func_like = {
        #[cfg(feature = "NEGATIVE")]
        {
            3
        }
        else {
            |a| a
        }
    };

    let y = func_like(3);

    if obj_like != 0.0 {
        std::process::exit(1);
    }
    if y != 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}