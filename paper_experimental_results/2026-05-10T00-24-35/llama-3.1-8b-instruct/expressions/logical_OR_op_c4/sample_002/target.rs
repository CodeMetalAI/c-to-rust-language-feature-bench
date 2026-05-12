// logical_OR_op_c4.rs

static mut G: i32 = 0;

fn set_g_return(gval: i32, ret: i32) -> i32 {
    unsafe {
        G = gval;
        return ret;
    }
}

fn main() {
    let mut y = 0;

    unsafe {
        G = 0;
        if !(set_g_return(1, 0) || (G == 1)) {
            return 1;
        }
    }

    unsafe {
        G = 0;
        if set_g_return(0, 1) || (++y) != 0 {
            return 2;
        }
        if y != 0 {
            return 3;
        }
    }

    return 0;
}