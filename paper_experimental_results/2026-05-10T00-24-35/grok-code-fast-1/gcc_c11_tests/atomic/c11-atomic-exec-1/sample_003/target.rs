use std::sync::atomic::*;
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Complex<T> {
    real: T,
    imag: T,
}

fn cmplx<T>(x: T, y: T) -> Complex<T> {
    Complex { real: x, imag: y }
}

fn abort() {
    std::process::abort();
}

macro_rules! test_atomic_assign {
    ($t:ty, $val:expr) => {
        let a = <$t>::new(0 as _);
        let b = <$t>::new($val as _);
        if a.load(Ordering::SeqCst) != 0 as _ { abort(); }
        if b.load(Ordering::SeqCst) != $val as _ { abort(); }
        let assigned = b.load(Ordering::SeqCst);
        a.store(assigned, Ordering::SeqCst);
        if a.load(Ordering::SeqCst) != $val as _ { abort(); }
    };
}

macro_rules! test_mutex_assign {
    ($t:ty, $val:expr, $zero:expr) => {
        let a = Mutex::new($zero);
        let b = Mutex::new($val);
        if *a.lock().unwrap() != $zero { abort(); }
        if *b.lock().unwrap() != $val { abort(); }
        let assigned = *b.lock().unwrap();
        *a.lock().unwrap() = assigned;
        if *a.lock().unwrap() != $val { abort(); }
    };
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct S {
    a: [i16; 1024],
}

fn test_simple_assign() {
    // TEST_SIMPLE_ASSIGN_ARITH(0)
    test_atomic_assign!(AtomicBool, false);
    test_atomic_assign!(AtomicI8, 0i8); // char
    test_atomic_assign!(AtomicI8, 0i8); // signed char
    test_atomic_assign!(AtomicU8, 0u8); // unsigned char
    test_atomic_assign!(AtomicI16, 0i16); // signed short
    test_atomic_assign!(AtomicU16, 0u16); // unsigned short
    test_atomic_assign!(AtomicI32, 0i32); // signed int
    test_atomic_assign!(AtomicU32, 0u32); // unsigned int
    test_atomic_assign!(AtomicIsize, 0isize); // signed long
    test_atomic_assign!(AtomicUsize, 0usize); // unsigned long
    test_atomic_assign!(AtomicI64, 0i64); // signed long long
    test_atomic_assign!(AtomicU64, 0u64); // unsigned long long
    test_mutex_assign!(f32, 0f32, 0f32); // float
    test_mutex_assign!(f64, 0f64, 0f64); // double
    test_mutex_assign!(f64, 0f64, 0f64); // long double
    test_mutex_assign!(Complex<f32>, Complex { real: 0f32, imag: 0f32 }, Complex { real: 0f32, imag: 0f32 }); // _Complex float
    test_mutex_assign!(Complex<f64>, Complex { real: 0f64, imag: 0f64 }, Complex { real: 0f64, imag: 0f64 }); // _Complex double
    test_mutex_assign!(Complex<f64>, Complex { real: 0f64, imag: 0f64 }, Complex { real: 0f64, imag: 0f64 }); // _Complex long double

    // TEST_SIMPLE_ASSIGN_ARITH(1)
    test_atomic_assign!(AtomicBool, true);
    test_atomic_assign!(AtomicI8, 1i8);
    test_atomic_assign!(AtomicI8, 1i8);
    test_atomic_assign!(AtomicU8, 1u8);
    test_atomic_assign!(AtomicI16, 1i16);
    test_atomic_assign!(AtomicU16, 1u16);
    test_atomic_assign!(AtomicI32, 1i32);
    test_atomic_assign!(AtomicU32, 1u32);
    test_atomic_assign!(AtomicIsize, 1isize);
    test_atomic_assign!(AtomicUsize, 1usize);
    test_atomic_assign!(AtomicI64, 1i64);
    test_atomic_assign!(AtomicU64, 1u64);
    test_mutex_assign!(f32, 1f32, 1f32);
    test_mutex_assign!(f64, 1f64, 1f64);
    test_mutex_assign!(f64, 1f64, 1f64);
    test_mutex_assign!(Complex<f32>, Complex { real: 1f32, imag: 0f32 }, Complex { real: 0f32, imag: 0f32 });
    test_mutex_assign!(Complex<f64>, Complex { real: 1f64, imag: 0f64 }, Complex { real: 0f64, imag: 0f64 });
    test_mutex_assign!(Complex<f64>, Complex { real: 1f64, imag: 0f64 }, Complex { real: 0f64, imag: 0f64 });

    // TEST_SIMPLE_ASSIGN_ARITH(2)
    test_atomic_assign!(AtomicBool, true);
    test_atomic_assign!(AtomicI8, 2i8);
    test_atomic_assign!(AtomicI8, 2i8);
    test_atomic_assign!(AtomicU8, 2u8);
    test_atomic_assign!(AtomicI16, 2i16);
    test_atomic_assign!(AtomicU16, 2u16);
    test_atomic_assign!(AtomicI32, 2i32);
    test_atomic_assign!(AtomicU32, 2u32);
    test_atomic_assign!(AtomicIsize, 2isize);
    test_atomic_assign!(AtomicUsize, 2usize);
    test_atomic_assign!(AtomicI64, 2i64);
    test_atomic_assign!(AtomicU64, 2u64);
    test_mutex_assign!(f32, 2f32, 2f32);
    test_mutex_assign!(f64, 2f64, 2f64);
    test_mutex_assign!(f64, 2f64, 2f64);
    test_mutex_assign!(Complex<f32>, Complex { real: 2f32, imag: 0f32 }, Complex { real: 0f32, imag: 0f32 });
    test_mutex_assign!(Complex<f64>, Complex { real: 2f64, imag: 0f64 }, Complex { real: 0f64, imag: 0f64 });
    test_mutex_assign!(Complex<f64>, Complex { real: 2f64, imag: 0f64 }, Complex { real: 0f64, imag: 0f64 });

    // TEST_SIMPLE_ASSIGN_ARITH(-1)
    test_atomic_assign!(AtomicBool, true);
    test_atomic_assign!(AtomicI8, -1i8);
    test_atomic_assign!(AtomicI8, -1i8);
    test_atomic_assign!(AtomicU8, -1i8 as u8);
    test_atomic_assign!(AtomicI16, -1i16);
    test_atomic_assign!(AtomicU16, -1i16 as u16);
    test_atomic_assign!(AtomicI32, -1i32);
    test_atomic_assign!(AtomicU32, -1i32 as u32);
    test_atomic_assign!(AtomicIsize, -1isize);
    test_atomic_assign!(AtomicUsize, -1isize as usize);
    test_atomic_assign!(AtomicI64, -1i64);
    test_atomic_assign!(AtomicU64, -1i64 as u64);
    test_mutex_assign!(f32, -1f32, -1f32);
    test_mutex_assign!(f64, -1f64, -1f64);
    test_mutex_assign!(f64, -1f64, -1f64);
    test_mutex_assign!(Complex<f32>, Complex { real: -1f32, imag: 0f32 }, Complex { real: 0f32, imag: 0f32 });
    test_mutex_assign!(Complex<f64>, Complex { real: -1f64, imag: 0f64 }, Complex { real: 0f64, imag: 0f64 });
    test_mutex_assign!(Complex<f64>, Complex { real: -1f64, imag: 0f64 }, Complex { real: 0f64, imag: 0f64 });

    // TEST_SIMPLE_ASSIGN_ARITH(1ULL << 63)
    let val_u64 = 1u64 << 63;
    test_atomic_assign!(AtomicBool, (val_u64 != 0));
    test_atomic_assign!(AtomicI8, val_u64 as i8);
    test_atomic_assign!(AtomicI8, val_u64 as i8);
    test_atomic_assign!(AtomicU8, val_u64 as u8);
    test_atomic_assign!(AtomicI16, val_u64 as i16);
    test_atomic_assign!(AtomicU16, val_u64 as u16);
    test_atomic_assign!(AtomicI32, val_u64 as i32);
    test_atomic_assign!(AtomicU32, val_u64 as u32);
    test_atomic_assign!(AtomicIsize, val_u64 as isize);
    test_atomic_assign!(AtomicUsize, val_u64 as usize);
    test_atomic_assign!(AtomicI64, val_u64 as i64);
    test_atomic_assign!(AtomicU64, val_u64);
    test_mutex_assign!(f32, val_u64 as f32, val_u64 as f32);
    test_mutex_assign!(f64, val_u64 as f64, val_u64 as f64);
    test_mutex_assign!(f64, val_u64 as f64, val_u64 as f64);
    test_mutex_assign!(Complex<f32>, Complex { real: val_u64 as f32, imag: 0f32 }, Complex { real: 0f32, imag: 0f32 });
    test_mutex_assign!(Complex<f64>, Complex { real: val_u64 as f64, imag: 0f64 }, Complex { real: 0f64, imag: 0f64 });
    test_mutex_assign!(Complex<f64>, Complex { real: val_u64 as f64, imag: 0f64 }, Complex { real: 0f64, imag: 0f64 });

    // TEST_SIMPLE_ASSIGN_ARITH(1.5)
    test_atomic_assign!(AtomicBool, true);
    test_atomic_assign!(AtomicI8, 1.5f64 as i8);
    test_atomic_assign!(AtomicI8, 1.5f64 as i8);
    test_atomic_assign!(AtomicU8, 1.5f64 as u8);
    test_atomic_assign!(AtomicI16, 1.5f64 as i16);
    test_atomic_assign!(AtomicU16, 1.5f64 as u16);
    test_atomic_assign!(AtomicI32, 1.5f64 as i32);
    test_atomic_assign!(AtomicU32, 1.5f64 as u32);
    test_atomic_assign!(AtomicIsize, 1.5f64 as isize);
    test_atomic_assign!(AtomicUsize, 1.5f64 as usize);
    test_atomic_assign!(AtomicI64, 1.5f64 as i64);
    test_atomic_assign!(AtomicU64, 1.5f64 as u64);
    test_mutex_assign!(f32, 1.5f32, 1.5f32);
    test_mutex_assign!(f64, 1.5f64, 1.5f64);
    test_mutex_assign!(f64, 1.5f64, 1.5f64);
    test_mutex_assign!(Complex<f32>, Complex { real: 1.5f32, imag: 0f32 }, Complex { real: 0f32, imag: 0f32 });
    test_mutex_assign!(Complex<f64>, Complex { real: 1.5f64, imag: 0f64 }, Complex { real: 0f64, imag: 0f64 });
    test_mutex_assign!(Complex<f64>, Complex { real: 1.5f64, imag: 0f64 }, Complex { real: 0f64, imag: 0f64 });

    // TEST_SIMPLE_ASSIGN_ARITH(CMPLX(2.5, 3.5))
    let cmplx_val = cmplx(2.5f64, 3.5f64);
    test_atomic_assign!(AtomicBool, true);
    test_atomic_assign!(AtomicI8, cmplx_val.real as i8);
    test_atomic_assign!(AtomicI8, cmplx_val.real as i8);
    test_atomic_assign!(AtomicU8, cmplx_val.real as u8);
    test_atomic_assign!(AtomicI16, cmplx_val.real as i16);
    test_atomic_assign!(AtomicU16, cmplx_val.real as u16);
    test_atomic_assign!(AtomicI32, cmplx_val.real as i32);
    test_atomic_assign!(AtomicU32, cmplx_val.real as u32);
    test_atomic_assign!(AtomicIsize, cmplx_val.real as isize);
    test_atomic_assign!(AtomicUsize, cmplx_val.real as usize);
    test_atomic_assign!(AtomicI64, cmplx_val.real as i64);
    test_atomic_assign!(AtomicU64, cmplx_val.real as u64);
    test_mutex_assign!(f32, cmplx_val.real as f32, cmplx_val.real as f32);
    test_mutex_assign!(f64, cmplx_val.real, cmplx_val.real);
    test_mutex_assign!(f64, cmplx_val.real, cmplx_val.real);
    test_mutex_assign!(Complex<f32>, cmplx_val as Complex<f32>, Complex { real: 0f32, imag: 0f32 });
    test_mutex_assign!(Complex<f64>, cmplx_val, Complex { real: 0f64, imag: 0f64 });
    test_mutex_assign!(Complex<f64>, cmplx_val, Complex { real: 0f64, imag: 0f64 });

    // Pointer tests
    static I: i32 = 0;
    let a_ptr = AtomicPtr::new(std::ptr::null_mut::<i32>());
    let b_ptr = AtomicPtr::new(std::ptr::null_mut::<i32>());
    if a_ptr.load(Ordering::SeqCst) != std::ptr::null() { abort(); }
    if b_ptr.load(Ordering::SeqCst) != std::ptr::null() { abort(); }
    let assigned_ptr = b_ptr.load(Ordering::SeqCst);
    a_ptr.store(assigned_ptr, Ordering::SeqCst);
    if a_ptr.load(Ordering::SeqCst) != std::ptr::null() { abort(); }

    let a_ptr2 = AtomicPtr::new(std::ptr::null_mut::<i32>());
    let b_ptr2 = AtomicPtr::new(&I as *const i32 as *mut i32);
    if a_ptr2.load(Ordering::SeqCst) != std::ptr::null() { abort(); }
    if b_ptr2.load(Ordering::SeqCst) != (&I as *const i32) { abort(); }
    let assigned_ptr2 = b_ptr2.load(Ordering::SeqCst);
    a_ptr2.store(assigned_ptr2, Ordering::SeqCst);
    if a_ptr2.load(Ordering::SeqCst) != (&I as *const i32) { abort(); }

    // Struct tests
    let s1 = Mutex::new(S { a: [0; 1024] });
    let s2 = Mutex::new(S { a: [0; 1024] });
    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut copy = init;
    *s1.lock().unwrap() = init;
    if copy != init { abort(); }
    let loaded = *s1.lock().unwrap();
    *s2.lock().unwrap() = loaded;
    copy = loaded;
    if copy != init { abort(); }
    copy = *s1.lock().unwrap();
    if copy != init { abort(); }
    copy = *s2.lock().unwrap();
    if copy != init { abort(); }
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}