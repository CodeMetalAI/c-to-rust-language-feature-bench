fn main() -> Result<(), i32> {
    let x = 1;
    let px = &x as *const _ as usize;

    let p2 = px as *const i32;

    if p2 as usize != px {
        return Err(1);
    }

    let z: usize = 1;
    let vp = z as *const ();
    let z2 = vp as usize;

    if z2 != z {
        return Err(2);
    }

    Ok(())
}