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
        };
        func_like(3)
    };

    if obj_like != 0.0 {
        return 1;
    }
    if func_like != 3 {
        return 2;
    }

    0
}