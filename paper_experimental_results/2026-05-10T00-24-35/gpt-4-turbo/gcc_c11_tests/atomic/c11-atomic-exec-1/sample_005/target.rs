use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicPtr};
use std::sync::atomic::Ordering::SeqCst;
use std::process;
use std::mem::size_of;
use std::ptr;

fn test_simple_assign<T: PartialEq + Clone + Copy + Default>(value: T) {
    let mut a = T::default();
    let mut b = value.clone();
    assert_eq!(a, T::default());
    assert_eq!(b, value);
    a = b.clone();
    assert_eq!(a, value);
    assert_eq!(a, value);
}

fn test_simple_assign_atomic<T: PartialEq + Copy + Default>(value: T, atomic_type: impl Fn(T) -> impl AtomicMethods<T>) {
    let a = atomic_type(T::default());
    let b = atomic_type(value);
    assert_eq!(a.load(), T::default());
    assert_eq!(b.load(), value);
    a.store(b.load());
    assert_eq!(a.load(), value);
    assert_eq!(a.load(), value);
}

trait AtomicMethods<T> {
    fn load(&self) -> T;
    fn store(&self, val: T);
}

impl AtomicMethods<bool> for AtomicBool {
    fn load(&self) -> bool { self.load(SeqCst) }
    fn store(&self, val: bool) { self.store(val, SeqCst) }
}

impl AtomicMethods<i8> for AtomicI8 {
    fn load(&self) -> i8 { self.load(SeqCst) }
    fn store(&self, val: i8) { self.store(val, SeqCst) }
}

impl AtomicMethods<u8> for AtomicU8 {
    fn load(&self) -> u8 { self.load(SeqCst) }
    fn store(&self, val: u8) { self.store(val, SeqCst) }
}

impl AtomicMethods<i16> for AtomicI16 {
    fn load(&self) -> i16 { self.load(SeqCst) }
    fn store(&self, val: i16) { self.store(val, SeqCst) }
}

impl AtomicMethods<u16> for AtomicU16 {
    fn load(&self) -> u16 { self.load(SeqCst) }
    fn store(&self, val: u16) { self.store(val, SeqCst) }
}

impl AtomicMethods<i32> for AtomicI32 {
    fn load(&self) -> i32 { self.load(SeqCst) }
    fn store(&self, val: i32) { self.store(val, SeqCst) }
}

impl AtomicMethods<u32> for AtomicU32 {
    fn load(&self) -> u32 { self.load(SeqCst) }
    fn store(&self, val: u32) { self.store(val, SeqCst) }
}

impl AtomicMethods<i64> for AtomicI64 {
    fn load(&self) -> i64 { self.load(SeqCst) }
    fn store(&self, val: i64) { self.store(val, SeqCst) }
}

impl AtomicMethods<u64> for AtomicU64 {
    fn load(&self) -> u64 { self.load(SeqCst) }
    fn store(&self, val: u64) { self.store(val, SeqCst) }
}

impl<T> AtomicMethods<*mut T> for AtomicPtr<T> {
    fn load(&self) -> *mut T { self.load(SeqCst) }
    fn store(&self, val: *mut T) { self.store(val, SeqCst) }
}

fn test_struct() {
    #[derive(Debug, Clone, Copy)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut s1 = init;
    let mut s2 = s1;

    assert_eq!(ptr::addr_of!(init), ptr::addr_of!(s1));  // Checks for equality of values
    assert_eq!(ptr::addr_of!(init), ptr::addr_of!(s2));
}

fn main() {
    test_simple_assign_atomic(false, AtomicBool::new);
    test_simple_assign_atomic(true, AtomicBool::new);
    test_simple_assign_atomic(1i8, AtomicI8::new);
    test_simple_assign_atomic(1u8, AtomicU8::new);
    test_simple_assign_atomic(1i16, AtomicI16::new);
    test_simple_assign_atomic(1u16, AtomicU16::new);
    test_simple_assign_atomic(1i32, AtomicI32::new);
    test_simple_assign_atomic(1u32, AtomicU32::new);
    test_simple_assign_atomic(1i64, AtomicI64::new);
    test_simple_assign_atomic(1u64, AtomicU64::new);
    test_simple_assign_atomic(ptr::null_mut::<i32>(), AtomicPtr::new);
    test_simple_assign_atomic(&mut 0 as *mut i32, AtomicPtr::new);
    test_struct();
    process::exit(0);
}