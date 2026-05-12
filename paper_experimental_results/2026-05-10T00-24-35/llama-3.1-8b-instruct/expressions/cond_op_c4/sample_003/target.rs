static mut g: i32 = 0;

fn set_g_return(gval: i32, ret: i32) -> i32 {
    unsafe {
        g = gval;
        ret
    }
}

fn main() {
    let mut x: i32;

    unsafe {
        g = 0;
        x = set_g_return(1, 1) as i32;
        if x!= 1 {
            return 1;
        }
        if g!= 1 {
            return 2;
        }
    }

    unsafe {
        g = 0;
        x = set_g_return(1, 0) as i32;
        if x!= 1 {
            return 3;
        }
        if g!= 1 {
            return 4;
        }
    }

    return 0;
}