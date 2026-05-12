fn main() {
    let x = {
        #[cfg(not(feature = "NEGATIVE"))]
        {
            (1 - 1)
        }
        #[cfg(feature = "NEGATIVE")]
        {
            0
        }
    };

    let y = {
        #[cfg(not(feature = "NEGATIVE"))]
        {
            |a| a
        }
        #[cfg(feature = "NEGATIVE")]
        {
            |b| b
        }
    }(3);

    if x != 0 {
        std::process::exit(1);
    }
    if y != 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}