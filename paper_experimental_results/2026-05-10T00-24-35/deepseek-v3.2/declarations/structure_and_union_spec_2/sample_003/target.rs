use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

fn main() {
    // Layout for the flexible array member equivalent
    // We'll manually manage the memory for the int + flexible double array

    // First, check that the offset of the flexible part is at the end of the base struct
    let base_layout = Layout::new::<i32>();
    let (layout, offset) = Layout::new::<i32>()
        .extend(Layout::array::<f64>(0).unwrap())
        .unwrap();

    // Check that the size of the struct with zero-length array is at least the size of i32
    if layout.size() < std::mem::size_of::<i32>() {
        std::process::exit(1);
    }

    // Check that the offset of the double array is at the end of the base struct
    if offset != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }

    // Allocate memory for s1 (base + 8 doubles)
    let s1_num_doubles = 8;
    let (s1_layout, s1_double_offset) = Layout::new::<i32>()
        .extend(Layout::array::<f64>(s1_num_doubles).unwrap())
        .unwrap();
    
    let s1_ptr = unsafe { alloc(s1_layout) };
    if s1_ptr.is_null() {
        std::process::exit(1);
    }

    // Allocate memory for s2 (base + 5 doubles)
    let s2_num_doubles = 5;
    let (s2_layout, s2_double_offset) = Layout::new::<i32>()
        .extend(Layout::array::<f64>(s2_num_doubles).unwrap())
        .unwrap();
    
    let s2_ptr = unsafe { alloc(s2_layout) };
    if s2_ptr.is_null() {
        unsafe { dealloc(s1_ptr, s1_layout) };
        std::process::exit(1);
    }

    // Write the integer part (n) if needed (not used in the test, but we'll keep it)
    unsafe {
        ptr::write(s1_ptr as *mut i32, 0);
        ptr::write(s2_ptr as *mut i32, 0);
    }

    // Set the double values
    unsafe {
        let s1_doubles_ptr = s1_ptr.add(s1_double_offset) as *mut f64;
        ptr::write(s1_doubles_ptr, 42.0);
        
        let s2_doubles_ptr = s2_ptr.add(s2_double_offset) as *mut f64;
        ptr::write(s2_doubles_ptr, 24.0);
    }

    // Check the values
    unsafe {
        let s1_doubles_ptr = s1_ptr.add(s1_double_offset) as *mut f64;
        let s2_doubles_ptr = s2_ptr.add(s2_double_offset) as *mut f64;
        
        if *s1_doubles_ptr != 42.0 || *s2_doubles_ptr != 24.0 {
            dealloc(s1_ptr, s1_layout);
            dealloc(s2_ptr, s2_layout);
            std::process::exit(1);
        }
    }

    // Deallocate
    unsafe {
        dealloc(s1_ptr, s1_layout);
        dealloc(s2_ptr, s2_layout);
    }

    std::process::exit(0);
}