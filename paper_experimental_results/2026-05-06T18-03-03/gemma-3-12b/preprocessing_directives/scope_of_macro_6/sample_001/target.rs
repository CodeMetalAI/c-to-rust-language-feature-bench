fn main() {
    let obj_like = {
        #[cfg(feature = "NEGATIVE")]
        {
            0.0
        }
        #[cfg(not(feature = "NEGATIVE"))]
        {
            (1.0 - 1.0)
        }
    };

    let func_like = {
        #[cfg(feature = "NEGATIVE")]
        {
            3
        }
        #[cfg(not(feature = "NEGATIVE"))]
        {
            3
        }
    };

    if obj_like != 0.0 {
        std::process::exit(1);
    }
    if func_like != 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}