use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::process;

fn main() {
    test_simple_assign();
    process::exit(0);
}

fn test_simple_assign() {
    // Arithmetic types with value 0
    test_simple_assign_bool(false);
    test_simple_assign_i8(0);
    test_simple_assign_i8(0);
    test_simple_assign_u8(0);
    test_simple_assign_i16(0);
    test_simple_assign_u16(0);
    test_simple_assign_i32(0);
    test_simple_assign_u32(0);
    test_simple_assign_isize(0);
    test_simple_assign_usize(0);
    test_simple_assign_i64(0);
    test_simple_assign_u64(0);
    test_simple_assign_f32(0.0);
    test_simple_assign_f64(0.0);
    test_simple_assign_f64(0.0);
    test_simple_assign_c32(0.0, 0.0);
    test_simple_assign_c64(0.0, 0.0);
    test_simple_assign_c64(0.0, 0.0);

    // Arithmetic types with value 1
    test_simple_assign_bool(true);
    test_simple_assign_i8(1);
    test_simple_assign_i8(1);
    test_simple_assign_u8(1);
    test_simple_assign_i16(1);
    test_simple_assign_u16(1);
    test_simple_assign_i32(1);
    test_simple_assign_u32(1);
    test_simple_assign_isize(1);
    test_simple_assign_usize(1);
    test_simple_assign_i64(1);
    test_simple_assign_u64(1);
    test_simple_assign_f32(1.0);
    test_simple_assign_f64(1.0);
    test_simple_assign_f64(1.0);
    test_simple_assign_c32(1.0, 0.0);
    test_simple_assign_c64(1.0, 0.0);
    test_simple_assign_c64(1.0, 0.0);

    // Arithmetic types with value 2
    test_simple_assign_bool(true);
    test_simple_assign_i8(2);
    test_simple_assign_i8(2);
    test_simple_assign_u8(2);
    test_simple_assign_i16(2);
    test_simple_assign_u16(2);
    test_simple_assign_i32(2);
    test_simple_assign_u32(2);
    test_simple_assign_isize(2);
    test_simple_assign_usize(2);
    test_simple_assign_i64(2);
    test_simple_assign_u64(2);
    test_simple_assign_f32(2.0);
    test_simple_assign_f64(2.0);
    test_simple_assign_f64(2.0);
    test_simple_assign_c32(2.0, 0.0);
    test_simple_assign_c64(2.0, 0.0);
    test_simple_assign_c64(2.0, 0.0);

    // Arithmetic types with value -1
    test_simple_assign_bool(true);
    test_simple_assign_i8(-1);
    test_simple_assign_i8(-1);
    test_simple_assign_u8((-1i8) as u8);
    test_simple_assign_i16(-1);
    test_simple_assign_u16((-1i16) as u16);
    test_simple_assign_i32(-1);
    test_simple_assign_u32((-1i32) as u32);
    test_simple_assign_isize(-1);
    test_simple_assign_usize((-1isize) as usize);
    test_simple_assign_i64(-1);
    test_simple_assign_u64((-1i64) as u64);
    test_simple_assign_f32(-1.0);
    test_simple_assign_f64(-1.0);
    test_simple_assign_f64(-1.0);
    test_simple_assign_c32(-1.0, 0.0);
    test_simple_assign_c64(-1.0, 0.0);
    test_simple_assign_c64(-1.0, 0.0);

    // Arithmetic types with value 1ULL << 63
    let val63 = 1u64 << 63;
    test_simple_assign_bool(true);
    test_simple_assign_i8(val63 as i8);
    test_simple_assign_i8(val63 as i8);
    test_simple_assign_u8(val63 as u8);
    test_simple_assign_i16(val63 as i16);
    test_simple_assign_u16(val63 as u16);
    test_simple_assign_i32(val63 as i32);
    test_simple_assign_u32(val63 as u32);
    test_simple_assign_isize(val63 as isize);
    test_simple_assign_usize(val63 as usize);
    test_simple_assign_i64(val63 as i64);
    test_simple_assign_u64(val63);
    test_simple_assign_f32(val63 as f32);
    test_simple_assign_f64(val63 as f64);
    test_simple_assign_f64(val63 as f64);
    test_simple_assign_c32(val63 as f32, 0.0);
    test_simple_assign_c64(val63 as f64, 0.0);
    test_simple_assign_c64(val63 as f64, 0.0);

    // Arithmetic types with value 1.5
    test_simple_assign_bool(true);
    test_simple_assign_i8(1);
    test_simple_assign_i8(1);
    test_simple_assign_u8(1);
    test_simple_assign_i16(1);
    test_simple_assign_u16(1);
    test_simple_assign_i32(1);
    test_simple_assign_u32(1);
    test_simple_assign_isize(1);
    test_simple_assign_usize(1);
    test_simple_assign_i64(1);
    test_simple_assign_u64(1);
    test_simple_assign_f32(1.5);
    test_simple_assign_f64(1.5);
    test_simple_assign_f64(1.5);
    test_simple_assign_c32(1.5, 0.0);
    test_simple_assign_c64(1.5, 0.0);
    test_simple_assign_c64(1.5, 0.0);

    // Arithmetic types with value CMPLX(2.5, 3.5)
    test_simple_assign_bool(true);
    test_simple_assign_i8(2);
    test_simple_assign_i8(2);
    test_simple_assign_u8(2);
    test_simple_assign_i16(2);
    test_simple_assign_u16(2);
    test_simple_assign_i32(2);
    test_simple_assign_u32(2);
    test_simple_assign_isize(2);
    test_simple_assign_usize(2);
    test_simple_assign_i64(2);
    test_simple_assign_u64(2);
    test_simple_assign_f32(2.5);
    test_simple_assign_f64(2.5);
    test_simple_assign_f64(2.5);
    test_simple_assign_c32(2.5, 3.5);
    test_simple_assign_c64(2.5, 3.5);
    test_simple_assign_c64(2.5, 3.5);

    // Pointer tests
    test_simple_assign_ptr_null();
    test_simple_assign_ptr_non_null();

    // Struct tests
    let mut init = SStruct { a: [0i16; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    let s1 = AtomicStruct::new(init);
    let copy = s1.load();
    if init != copy {
        process::abort();
    }

    let s2 = AtomicStruct::new(copy);
    let copy = s2.load();
    if init != copy {
        process::abort();
    }

    let copy = s1.load();
    if init != copy {
        process::abort();
    }

    let copy = s2.load();
    if init != copy {
        process::abort();
    }
}

fn test_simple_assign_bool(value: bool) {
    let a = AtomicBool::new(false);
    let b = AtomicBool::new(value);
    if a.load(Ordering::SeqCst) != false {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let result = b.load(Ordering::SeqCst);
    a.store(result, Ordering::SeqCst);
    if result != value {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_simple_assign_i8(value: i8) {
    let a = AtomicI8::new(0);
    let b = AtomicI8::new(value);
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let result = b.load(Ordering::SeqCst);
    a.store(result, Ordering::SeqCst);
    if result != value {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_simple_assign_u8(value: u8) {
    let a = AtomicU8::new(0);
    let b = AtomicU8::new(value);
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let result = b.load(Ordering::SeqCst);
    a.store(result, Ordering::SeqCst);
    if result != value {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_simple_assign_i16(value: i16) {
    let a = AtomicI16::new(0);
    let b = AtomicI16::new(value);
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let result = b.load(Ordering::SeqCst);
    a.store(result, Ordering::SeqCst);
    if result != value {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_simple_assign_u16(value: u16) {
    let a = AtomicU16::new(0);
    let b = AtomicU16::new(value);
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let result = b.load(Ordering::SeqCst);
    a.store(result, Ordering::SeqCst);
    if result != value {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_simple_assign_i32(value: i32) {
    let a = AtomicI32::new(0);
    let b = AtomicI32::new(value);
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let result = b.load(Ordering::SeqCst);
    a.store(result, Ordering::SeqCst);
    if result != value {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_simple_assign_u32(value: u32) {
    let a = AtomicU32::new(0);
    let b = AtomicU32::new(value);
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let result = b.load(Ordering::SeqCst);
    a.store(result, Ordering::SeqCst);
    if result != value {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_simple_assign_isize(value: isize) {
    let a = AtomicIsize::new(0);
    let b = AtomicIsize::new(value);
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let result = b.load(Ordering::SeqCst);
    a.store(result, Ordering::SeqCst);
    if result != value {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_simple_assign_usize(value: usize) {
    let a = AtomicUsize::new(0);
    let b = AtomicUsize::new(value);
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let result = b.load(Ordering::SeqCst);
    a.store(result, Ordering::SeqCst);
    if result != value {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_simple_assign_i64(value: i64) {
    let a = AtomicI64::new(0);
    let b = AtomicI64::new(value);
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let result = b.load(Ordering::SeqCst);
    a.store(result, Ordering::SeqCst);
    if result != value {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_simple_assign_u64(value: u64) {
    let a = AtomicU64::new(0);
    let b = AtomicU64::new(value);
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let result = b.load(Ordering::SeqCst);
    a.store(result, Ordering::SeqCst);
    if result != value {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_simple_assign_f32(value: f32) {
    use std::sync::Mutex;
    let a = Mutex::new(0.0f32);
    let b = Mutex::new(value);
    if *a.lock().unwrap() != 0.0 {
        process::abort();
    }
    if *b.lock().unwrap() != value {
        process::abort();
    }
    let result = *b.lock().unwrap();
    *a.lock().unwrap() = result;
    if result != value {
        process::abort();
    }
    if *a.lock().unwrap() != value {
        process::abort();
    }
}

fn test_simple_assign_f64(value: f64) {
    use std::sync::Mutex;
    let a = Mutex::new(0.0f64);
    let b = Mutex::new(value);
    if *a.lock().unwrap() != 0.0 {
        process::abort();
    }
    if *b.lock().unwrap() != value {
        process::abort();
    }
    let result = *b.lock().unwrap();
    *a.lock().unwrap() = result;
    if result != value {
        process::abort();
    }
    if *a.lock().unwrap() != value {
        process::abort();
    }
}

fn test_simple_assign_c32(real: f32, imag: f32) {
    use std::sync::Mutex;
    let a = Mutex::new((0.0f32, 0.0f32));
    let b = Mutex::new((real, imag));
    if *a.lock().unwrap() != (0.0, 0.0) {
        process::abort();
    }
    if *b.lock().unwrap() != (real, imag) {
        process::abort();
    }
    let result = *b.lock().unwrap();
    *a.lock().unwrap() = result;
    if result != (real, imag) {
        process::abort();
    }
    if *a.lock().unwrap() != (real, imag) {
        process::abort();
    }
}

fn test_simple_assign_c64(real: f64, imag: f64) {
    use std::sync::Mutex;
    let a = Mutex::new((0.0f64, 0.0f64));
    let b = Mutex::new((real, imag));
    if *a.lock().unwrap() != (0.0, 0.0) {
        process::abort();
    }
    if *b.lock().unwrap() != (real, imag) {
        process::abort();
    }
    let result = *b.lock().unwrap();
    *a.lock().unwrap() = result;
    if result != (real, imag) {
        process::abort();
    }
    if *a.lock().unwrap() != (real, imag) {
        process::abort();
    }
}

fn test_simple_assign_ptr_null() {
    let a = AtomicUsize::new(0);
    let b = AtomicUsize::new(0);
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    let result = b.load(Ordering::SeqCst);
    a.store(result, Ordering::SeqCst);
    if result != 0 {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
}

fn test_simple_assign_ptr_non_null() {
    static mut I: i32 = 0;
    let ptr_val = unsafe { &I as *const i32 as usize };
    let a = AtomicUsize::new(0);
    let b = AtomicUsize::new(ptr_val);
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != ptr_val {
        process::abort();
    }
    let result = b.load(Ordering::SeqCst);
    a.store(result, Ordering::SeqCst);
    if result != ptr_val {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != ptr_val {
        process::abort();
    }
}

#[derive(Clone, Copy, PartialEq)]
struct SStruct {
    a: [i16; 1024],
}

struct AtomicStruct {
    inner: std::sync::Mutex<SStruct>,
}

impl AtomicStruct {
    fn new(val: SStruct) -> Self {
        AtomicStruct {
            inner: std::sync::Mutex::new(val),
        }
    }

    fn load(&self) -> SStruct {
        *self.inner.lock().unwrap()
    }
}