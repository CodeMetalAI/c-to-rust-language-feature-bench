fn f(init: i32) -> i32 {
    let mut x = init;
    let mut p = &mut x;

    // Create a mutable array of 2 integers
    let mut arr = [0i32; 2];
    arr[0] = *p;
    p = &mut arr[0];

    // Convert to a slice of i32s.  This is needed to dereference
    // and assign a value to the array element.
    let ptr: *mut i32 = p;
    unsafe {
      *ptr = init;
    }

    let mut arr2 = [0i32; 2];
    unsafe {
      arr2[0] = *ptr;
    }
    p = &mut arr2[0];
    
    let mut arr3 = [0i32; 2];
    unsafe {
        arr3[0] = *p;
    }
    
    let mut arr4 = [0i32; 2];
    unsafe {
        arr4[0] = *p;
    }
    

    if arr2[0] != init {
        return 1;
    }
    if arr2[1] != 0 {
        return 2;
    }

    0
}

fn main() {
    if f(7) != 0 {
        std::process::exit(10);
    }
    if f(-3) != 0 {
        std::process::exit(11);
    }
    std::process::exit(0);
}