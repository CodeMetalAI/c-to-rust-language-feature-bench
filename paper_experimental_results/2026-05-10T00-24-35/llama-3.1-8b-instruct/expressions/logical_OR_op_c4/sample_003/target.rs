fn main() {
    let mut g: i32 = 0;
    let mut y: i32 = 0;

    // Test case 1
    g = 0;
    let ret = if set_g_return(1, 0) || g == 1 { 1 } else { 0 };
    println!("{}", ret);

    // Test case 2
    g = 0;
    let ret = if set_g_return(0, 1) || increment_and_check(y) { 2 } else { 0 };
    println!("{}", ret);

    // Test case 3
    let ret = if y != 0 { 3 } else { 0 };
    println!("{}", ret);
}

fn set_g_return(gval: i32, ret: i32) -> i32 {
    g = gval;
    ret
}

fn increment_and_check(y: i32) -> bool {
    y += 1;
    y != 0
}