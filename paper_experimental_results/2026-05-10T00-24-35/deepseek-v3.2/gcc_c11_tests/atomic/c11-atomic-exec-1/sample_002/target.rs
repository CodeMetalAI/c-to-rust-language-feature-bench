use std::mem;

fn abort() {
    std::process::exit(1);
}

fn exit(code: i32) {
    std::process::exit(code);
}

fn test_simple_assign() {
    // Test integer types
    test_simple_assign_integer(0);
    test_simple_assign_integer(1);
    test_simple_assign_integer(2);
    test_simple_assign_integer(-1);
    test_simple_assign_integer(1u64 << 63);

    // Test floating point types
    test_simple_assign_floating(0.0);
    test_simple_assign_floating(1.0);
    test_simple_assign_floating(2.0);
    test_simple_assign_floating(-1.0);
    test_simple_assign_floating(1.5);

    // Test complex types (using tuples to simulate complex)
    test_simple_assign_complex((0.0, 0.0));
    test_simple_assign_complex((1.0, 0.0));
    test_simple_assign_complex((2.0, 0.0));
    test_simple_assign_complex((-1.0, 0.0));
    test_simple_assign_complex((2.5, 3.5));

    // Test pointers
    let mut i = 0;
    test_simple_assign_pointer(std::ptr::null());
    test_simple_assign_pointer(&mut i);

    // Test struct
    struct S {
        a: [i16; 1024],
    }
    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    
    let s1 = std::sync::atomic::AtomicPtr::new(&init);
    let copy_ptr = s1.load(std::sync::atomic::Ordering::SeqCst);
    let copy = unsafe { &*copy_ptr };
    if mem::ne(&init.a, &copy.a) {
        abort();
    }
    
    let s2 = std::sync::atomic::AtomicPtr::new(copy_ptr);
    let copy_ptr2 = s2.load(std::sync::atomic::Ordering::SeqCst);
    let copy2 = unsafe { &*copy_ptr2 };
    if mem::ne(&init.a, &copy2.a) {
        abort();
    }
}

fn test_simple_assign_integer<T: Eq + Copy>(value: T) {
    use std::sync::atomic::Atomic;
    
    let a = Atomic::new(T::default());
    let b = Atomic::new(value);
    
    if a.load(std::sync::atomic::Ordering::SeqCst) != T::default() {
        abort();
    }
    if b.load(std::sync::atomic::Ordering::SeqCst) != value {
        abort();
    }
    
    a.store(b.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
    if a.load(std::sync::atomic::Ordering::SeqCst) != value {
        abort();
    }
}

fn test_simple_assign_floating<T: PartialEq + Copy>(value: T) {
    use std::sync::atomic::Atomic;
    
    let a = Atomic::new(T::default());
    let b = Atomic::new(value);
    
    if a.load(std::sync::atomic::Ordering::SeqCst) != T::default() {
        abort();
    }
    if b.load(std::sync::atomic::Ordering::SeqCst) != value {
        abort();
    }
    
    a.store(b.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
    if a.load(std::sync::atomic::Ordering::SeqCst) != value {
        abort();
    }
}

fn test_simple_assign_complex<T: PartialEq + Copy>(value: T) {
    use std::sync::atomic::Atomic;
    
    let a = Atomic::new(T::default());
    let b = Atomic::new(value);
    
    if a.load(std::sync::atomic::Ordering::SeqCst) != T::default() {
        abort();
    }
    if b.load(std::sync::atomic::Ordering::SeqCst) != value {
        abort();
    }
    
    a.store(b.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
    if a.load(std::sync::atomic::Ordering::SeqCst) != value {
        abort();
    }
}

fn test_simple_assign_pointer<T>(value: *mut T) {
    use std::sync::atomic::AtomicPtr;
    
    let a = AtomicPtr::new(std::ptr::null_mut());
    let b = AtomicPtr::new(value);
    
    if a.load(std::sync::atomic::Ordering::SeqCst) != std::ptr::null_mut() {
        abort();
    }
    if b.load(std::sync::atomic::Ordering::SeqCst) != value {
        abort();
    }
    
    a.store(b.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
    if a.load(std::sync::atomic::Ordering::SeqCst) != value {
        abort();
    }
}

fn main() {
    test_simple_assign();
    exit(0);
}