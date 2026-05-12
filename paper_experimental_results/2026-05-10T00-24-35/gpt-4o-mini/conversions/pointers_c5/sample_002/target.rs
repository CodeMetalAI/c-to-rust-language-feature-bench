fn main() -> Result<(), i32> {
    let x = 1;
    let px = &x as *const _ as usize; // Convert to usize (equivalent to uintptr_t)

    let p2 = px as *const i32; // Cast back to pointer
    if p2 as usize != px {
        return Err(1);
    }

    let z: usize = 1; // Using usize to represent uintptr_t
    let vp = z as *const (); // Cast to void*
    let z2 = vp as usize; // Convert back to usize

    if z2 != z {
        return Err(2);
    }

    Ok(())
}