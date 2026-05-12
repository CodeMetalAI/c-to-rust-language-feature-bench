static mut G: i32 = 0;

fn set_g_return(gval: i32, ret: i32) -> i32 {
    unsafe {
        G = gval;
        ret
    }
}

fn main() {
    let x;

    unsafe {
        G = 0;
        x = (set_g_return(1, 1) as u8 == 1) as i32 ? (G == 1) : (set_g_return(2, 1), 0);
        if x != 1 {
            return 1;
        }
        if G != 1 {
            return 2;
        }
    }

    unsafe {
        G = 0;
        x = (set_g_return(1, 0) as u8 == 1) as i32 ? (set_g_return(2, 1), 0) : (G == 1);
        if x != 1 {
            return 3;
        }
        if G != 1 {
            return 4;
        }
    }

    0
}