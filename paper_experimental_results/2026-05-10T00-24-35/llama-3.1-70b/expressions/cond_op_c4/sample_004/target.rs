fn set_g_return(gval: i32, ret: i32) -> i32 {
    static mut G: i32 = 0;

    unsafe {
        G = gval;
    }
    ret
}

fn main() {
    static mut G: i32 = 0;

    unsafe {
        G = 0;
        let x = if set_g_return(1, 1)!= 0 {
            G == 1
        } else {
            set_g_return(2, 1);
            0
        };
        if x!= 1 {
            return 1;
        }
        if G!= 1 {
            return 2;
        }

        G = 0;
        let x = if set_g_return(1, 0)!= 0 {
            set_g_return(2, 1);
            0
        } else {
            G == 1
        };
        if x!= 1 {
            return 3;
        }
        if G!= 1 {
            return 4;
        }
    }

    0
}