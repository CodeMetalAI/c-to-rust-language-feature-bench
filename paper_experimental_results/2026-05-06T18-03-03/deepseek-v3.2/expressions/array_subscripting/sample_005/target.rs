use std::ptr;

fn main() {
    let mut x = [[0; 5]; III];

    for i in 0..III {
        for j in 0..5 {
            x[i][j] = 1 * i + j;
        }
    }

    for i in 0..III {
        for j in 0..5 {
            let a = x[i][j];
            let b = unsafe { *(*x.as_ptr().add(i)).add(j) };
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0 = &x[0][0] as *const i32;
    let p1 = &x[1][0] as *const i32;
    if unsafe { p1.offset_from(p0) } != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}