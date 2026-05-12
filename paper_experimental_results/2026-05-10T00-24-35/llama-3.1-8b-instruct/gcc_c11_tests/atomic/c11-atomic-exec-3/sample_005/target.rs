use std::sync::atomic::{AtomicIsize, Ordering};
use std::cell::RefCell;

fn test_incdec_arith(value: i32) {
    let a = AtomicIsize::new(value);

    for op in ["++", "--", "+=", "-=", "++", "--"] {
        let preop = if op == "++" || op == "--" { true } else { false };
        let postop = if op == "++" || op == "--" { true } else { false };
        let change = if op == "++" { 1 } else if op == "--" { -1 } else { 1 };

        for ty in ["i32", "i64", "f32", "f64", "usize", "isize", "u64", "i16", "u16", "i8", "u8", "i32", "u32", "i64", "u64", "i128", "u128"] {
            let ty = match ty {
                "i32" => AtomicIsize::new(value as _),
                "i64" => AtomicIsize::new(value as _),
                "f32" => AtomicF32::new(value as _),
                "f64" => AtomicF64::new(value as _),
                "usize" => AtomicUsize::new(value as _),
                "isize" => AtomicIsize::new(value as _),
                "u64" => AtomicUsize::new(value as _),
                "i16" => AtomicIsize::new(value as _),
                "u16" => AtomicUsize::new(value as _),
                "i8" => AtomicIsize::new(value as _),
                "u8" => AtomicUsize::new(value as _),
                "i32" => AtomicIsize::new(value as _),
                "u32" => AtomicUsize::new(value as _),
                "i64" => AtomicIsize::new(value as _),
                "u64" => AtomicUsize::new(value as _),
                "i128" => AtomicIsize::new(value as _),
                "u128" => AtomicUsize::new(value as _),
                _ => panic!("Invalid type"),
            };
            if op != "" {
                if ty.fetch_add(change, Ordering::SeqCst) != value + change {
                    panic!("fetch_add failed");
                }
                if ty.load(Ordering::SeqCst) != value + change {
                    panic!("load failed");
                }
            } else {
                if ty.fetch_sub(change, Ordering::SeqCst) != value + change {
                    panic!("fetch_sub failed");
                }
                if ty.load(Ordering::SeqCst) != value + change {
                    panic!("load failed");
                }
            }
        }
    }
}

fn test_all_incdec_arith(value: i32) {
    test_incdec_arith(value);
    test_incdec_arith(value + 1);
    test_incdec_arith(value - 1);
}

fn test_incdec_ptr<T: Clone + Copy + std::sync::atomic::Sync + std::marker::Copy + std::marker::Sync>(value: T) {
    let ia = [0; 2];
    let ptr = &mut ia[1] as *mut T;
    let a = AtomicPtr::new(ptr);

    for op in ["++", "--", "+=", "-=", "++", "--"] {
        let preop = if op == "++" || op == "--" { true } else { false };
        let postop = if op == "++" || op == "--" { true } else { false };
        let change = if op == "++" { 1 } else if op == "--" { -1 } else { 1 };

        let ty = match op {
            "++" | "--" => AtomicPtr::new(ptr),
            _ => AtomicPtr::new(ptr),
        };
        if op != "" {
            if ty.fetch_add(change, Ordering::SeqCst) != ptr.offset(change) {
                panic!("fetch_add failed");
            }
            if *ty.load(Ordering::SeqCst) != ptr.offset(change) {
                panic!("load failed");
            }
        } else {
            if ty.fetch_sub(change, Ordering::SeqCst) != ptr.offset(change) {
                panic!("fetch_sub failed");
            }
            if *ty.load(Ordering::SeqCst) != ptr.offset(change) {
                panic!("load failed");
            }
        }
    }
}

struct RefGuard<'a, T: ?Sized> {
    inner: &'a T,
}

impl<'a, T: ?Sized + Sync + Send> Drop for RefGuard<'a, T> {
    fn drop(&mut self) {
        std::mem::drop(self.inner);
    }
}

fn test_ptr(value: i32) {
    let mut ia = [0; 2];
    let ptr = &mut ia[1] as *mut i32;
    let a = RefCell::new(ptr);

    for op in ["++", "--", "+=", "-=", "++", "--"] {
        let preop = if op == "++" || op == "--" { true } else { false };
        let postop = if op == "++" || op == "--" { true } else { false };
        let change = if op == "++" { 1 } else if op == "--" { -1 } else { 1 };

        let ty = match op {
            "++" | "--" => ptr,
            _ => ptr,
        };
        if op != "" {
            let _guard = RefGuard { inner: a.borrow_mut() };
            if unsafe { *ty.offset(change) } != ptr.offset(change) {
                panic!("fetch_add failed");
            }
            if unsafe { *ty.load(*std::ptr::read(unsafe { ptr.offset(change) })) } != ptr.offset(change) {
                panic!("load failed");
            }
        } else {
            let _guard = RefGuard { inner: a.borrow_mut() };
            if unsafe { *ty.offset(change) } != ptr.offset(change) {
                panic!("fetch_sub failed");
            }
            if unsafe { *ty.load(*std::ptr::read(unsafe { ptr.offset(change) })) } != ptr.offset(change) {
                panic!("load failed");
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        test_incdec_arith(0);
        test_incdec_arith(1);
        test_incdec_arith(2);
        test_incdec_arith(-1);
        test_incdec_arith(1i64 << 60);
        test_incdec_arith(1.5);
        test_ptr(0);
        test_ptr(1);
    }
}

fn main() {
    test_incdec_arith(0);
    test_incdec_arith(1);
    test_incdec_arith(2);
    test_incdec_arith(-1);
    test_incdec_arith(1i64 << 60);
    test_incdec_arith(1.5);
    test_ptr(0);
    test_ptr(1);
    std::process::exit(0);
}