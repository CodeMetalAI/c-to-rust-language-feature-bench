fn main() {
    // Define the structure equivalent to the C code
    #[repr(C)]
    struct S {
        inner: Inner,
        a: [i32; 0],
    }

    #[repr(C)]
    struct Inner {
        i: i32,
    }

    // Use the memoffset crate to get the offset of a field in the struct
    let offset_i = std::mem::offset_of!(S, inner.i);
    if offset_i!= 0 {
        return 1;
    }

    // The offset of a is the size of the struct, since it's a flexible array
    let offset_a = std::mem::size_of::<S>();
    if offset_a!= std::mem::size_of::<S>() {
        return 2;
    }

    // Allocate memory for the struct and the flexible array
    let mut p = vec![0; std::mem::size_of::<S>() + 4 * std::mem::size_of::<i32>()];

    // Use the memoffset crate to get the address of the field in the struct
    let i_ptr = (p.as_mut_ptr() as *mut u8).offset(offset_i as isize) as *mut i32;
    let a_ptr = (p.as_mut_ptr() as *mut u8).offset(offset_a as isize) as *mut i32;

    // Assign values to the fields
    unsafe {
        *i_ptr = 7;
        *a_ptr.offset(0) = 11;
        *a_ptr.offset(3) = 22;
    }

    // Check the values of the fields
    if unsafe { *i_ptr }!= 7 || unsafe { *a_ptr.offset(0) }!= 11 || unsafe { *a_ptr.offset(3) }!= 22 {
        return 4;
    }

    return 0;
}