static mut G: i32 = 0;

fn set_g_return(gval: i32, ret: i32) -> i32 {
    G = gval;
    ret
}

fn main() -> i32 {
    let mut y = 0;

    G = 0;
    let cond = set_g_return(0, 0) && (y + 1);
    if cond {
        1
    } else if y!= 0 {
        2
    } else {
        0
    };

    G = 0;
    let cond = set_g_return(1, 1)!= (G == 1);
    if cond {
        3
    } else {
        0
    }
}