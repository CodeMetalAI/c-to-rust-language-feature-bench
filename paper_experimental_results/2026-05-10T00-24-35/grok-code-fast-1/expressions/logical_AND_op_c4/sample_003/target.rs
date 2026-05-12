use std::process;

fn set_g_return(g: &mut i32, gval: i32, ret: i32) -> i32 {
    *g = gval;
    ret
}

fn main() {
    let mut g = 0;
    let mut y = 0;

    g = 0;
    if (set_g_return(&mut g, 0, 0) != 0) && ({ y += 1; y } != 0) {
        process::exit(1);
    }
    if y != 0 {
        process::exit(2);
    }

    g = 0;
    if !((set_g_return(&mut g, 1, 1) != 0) && (g == 1)) {
        process::exit(3);
    }

    process::exit(0);
}