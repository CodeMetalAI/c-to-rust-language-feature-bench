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

    let func_like = |a: i32| -> i32 {
        #[cfg(feature = "NEGATIVE")]
        {
            a
        }
        #[cfg(not(feature = "NEGATIVE"))]
        {
            a
        }
    };

    let x = obj_like as i32;
    let y = func_like(3);

    if x != 0 {
        std::process::exit(1);
    }
    if y != 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}