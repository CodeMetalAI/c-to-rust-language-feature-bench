use std::process::exit;

fn main() {
    let x: i32 = 1;
    let px: &i32 = &x;

    let ip: usize = px as *const i32 as usize;
    let p2: *const i32 = ip as *const i32;

    if (p2 as usize) != ip {
        exit(1);
    }

    let z: usize = 1;
    let vp: *const () = z as *const ();
    let z2: usize = vp as usize;

    if z2 != z {
        exit(2);
    }

    exit(0);
}