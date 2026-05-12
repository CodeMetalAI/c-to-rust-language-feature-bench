fn main() -> i32 {
    let x: i32 = 1;
    let px = &x;

    let ip: usize = px as usize;
    let p2: *const i32 = ip as *const i32;

    if ip as *const i32!= p2 {
        return 1;
    }

    let z: usize = 1;
    let vp: *const () = z as *const ();
    let z2: usize = vp as usize;

    if z2!= z {
        return 2;
    }

    0
}