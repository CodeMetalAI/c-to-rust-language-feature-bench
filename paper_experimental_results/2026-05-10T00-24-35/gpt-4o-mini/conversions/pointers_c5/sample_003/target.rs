fn main() -> Result<(), i32> {
    let x = 1;
    let px = &x as *const _ as usize; // cast to usize (similar to uintptr_t)

    let p2 = px as *const i32; // reinterpret as pointer to i32

    if p2 as usize != px {
        return Err(1);
    }

    let z: usize = 1; // equivalent to uintptr_t
    let vp = z as *const (); // cast to void pointer (similar to void *)
    let z2 = vp as usize; // cast back to uintptr_t

    if z2 != z {
        return Err(2);
    }

    Ok(())
}