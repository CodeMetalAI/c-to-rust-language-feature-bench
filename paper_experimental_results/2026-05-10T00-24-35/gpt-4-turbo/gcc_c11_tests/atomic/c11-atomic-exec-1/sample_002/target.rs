fn main() {
    test_simple_assign();
    std::process::exit(0);
}

fn test_simple_assign() {
    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1u64 << 63);
    test_simple_assign_arith(1.5);
    test_simple_assign_arith(Complex::new(2.5, 3.5));
    static mut I: i32 = 0;
    test_simple_assign_generic::<*const i32>(std::ptr::null());
    unsafe {
        test_simple_assign_generic(&I as *const i32);
    }
    let mut init = S { a: [0; 1024] };
    let mut copy: S;
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let s1 = std::sync::atomic::AtomicPtr::new(&mut init as *mut S);
    let s2 = std::sync::atomic::AtomicPtr::new(s1.load(std::sync::atomic::Ordering::SeqCst));
    copy = unsafe { *s1.load(std::sync::atomic::Ordering::SeqCst) };
    assert_eq!(&init, &copy);
    copy = unsafe { *s2.load(std::sync::atomic::Ordering::SeqCst) };
    assert_eq!(&init, &copy);
    copy = unsafe { *s1.load(std::sync::atomic::Ordering::SeqCst) };
    assert_eq!(&init, &copy);
    copy = unsafe { *s2.load(std::sync::atomic::Ordering::SeqCst) };
    assert_eq!(&init, &copy);
}

fn test_simple_assign_arith<T: PartialEq + Copy + From<u8> + std::fmt::Debug>(value: T) {
    test_simple_assign_generic(value);
}

fn test_simple_assign_generic<T: PartialEq + Copy + std::fmt::Debug>(value: T) {
    let a = std::sync::atomic::Atomic::new(T::from(0));
    let b = std::sync::atomic::Atomic::new(value);
    assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), T::from(0));
    assert_eq!(b.load(std::sync::atomic::Ordering::SeqCst), value);
    a.store(b.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
    assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), value);
}

#[derive(PartialEq, Debug)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
}

#[derive(PartialEq, Debug)]
struct S {
    a: [i16; 1024],
}