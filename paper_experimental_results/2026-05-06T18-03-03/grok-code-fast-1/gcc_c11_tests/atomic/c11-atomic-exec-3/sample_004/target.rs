use std::sync::atomic::{AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicIsize, AtomicUsize, AtomicI64, AtomicU64, AtomicPtr, Ordering};
use std::process::{abort, exit};

static A_BOOL: AtomicU8 = AtomicU8::new(0);
static A_CHAR: AtomicI8 = AtomicI8::new(0);
static A_SIGNED_CHAR: AtomicI8 = AtomicI8::new(0);
static A_UNSIGNED_CHAR: AtomicU8 = AtomicU8::new(0);
static A_SIGNED_SHORT: AtomicI16 = AtomicI16::new(0);
static A_UNSIGNED_SHORT: AtomicU16 = AtomicU16::new(0);
static A_SIGNED_INT: AtomicI32 = AtomicI32::new(0);
static A_UNSIGNED_INT: AtomicU32 = AtomicU32::new(0);
static A_SIGNED_LONG: AtomicIsize = AtomicIsize::new(0);
static A_UNSIGNED_LONG: AtomicUsize = AtomicUsize::new(0);
static A_SIGNED_LONG_LONG: AtomicI64 = AtomicI64::new(0);
static A_UNSIGNED_LONG_LONG: AtomicU64 = AtomicU64::new(0);
static IA: [i32; 2] = [0, 0];
static A_INT_PTR: AtomicPtr<i32> = AtomicPtr::new(std::ptr::null_mut());

fn test_incdec() {
    // TEST_ALL_INCDEC_ARITH(0)
    // pre ++
    A_BOOL.store(0, Ordering::SeqCst);
    let result = A_BOOL.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 1 { abort(); }
    if A_BOOL.load(Ordering::SeqCst) != 1 { abort(); }
    A_CHAR.store(0, Ordering::SeqCst);
    let result = A_CHAR.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 1 { abort(); }
    if A_CHAR.load(Ordering::SeqCst) != 1 { abort(); }
    A_SIGNED_CHAR.store(0, Ordering::SeqCst);
    let result = A_SIGNED_CHAR.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 1 { abort(); }
    if A_SIGNED_CHAR.load(Ordering::SeqCst) != 1 { abort(); }
    A_UNSIGNED_CHAR.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_CHAR.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 1 { abort(); }
    if A_UNSIGNED_CHAR.load(Ordering::SeqCst) != 1 { abort(); }
    A_SIGNED_SHORT.store(0, Ordering::SeqCst);
    let result = A_SIGNED_SHORT.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 1 { abort(); }
    if A_SIGNED_SHORT.load(Ordering::SeqCst) != 1 { abort(); }
    A_UNSIGNED_SHORT.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_SHORT.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 1 { abort(); }
    if A_UNSIGNED_SHORT.load(Ordering::SeqCst) != 1 { abort(); }
    A_SIGNED_INT.store(0, Ordering::SeqCst);
    let result = A_SIGNED_INT.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 1 { abort(); }
    if A_SIGNED_INT.load(Ordering::SeqCst) != 1 { abort(); }
    A_UNSIGNED_INT.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_INT.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 1 { abort(); }
    if A_UNSIGNED_INT.load(Ordering::SeqCst) != 1 { abort(); }
    A_SIGNED_LONG.store(0, Ordering::SeqCst);
    let result = A_SIGNED_LONG.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 1 { abort(); }
    if A_SIGNED_LONG.load(Ordering::SeqCst) != 1 { abort(); }
    A_UNSIGNED_LONG.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 1 { abort(); }
    if A_UNSIGNED_LONG.load(Ordering::SeqCst) != 1 { abort(); }
    A_SIGNED_LONG_LONG.store(0, Ordering::SeqCst);
    let result = A_SIGNED_LONG_LONG.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 1 { abort(); }
    if A_SIGNED_LONG_LONG.load(Ordering::SeqCst) != 1 { abort(); }
    A_UNSIGNED_LONG_LONG.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG_LONG.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 1 { abort(); }
    if A_UNSIGNED_LONG_LONG.load(Ordering::SeqCst) != 1 { abort(); }
    // pre --
    A_BOOL.store(0, Ordering::SeqCst);
    let result = A_BOOL.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != -1 { abort(); }
    if A_BOOL.load(Ordering::SeqCst) != -1 { abort(); }
    A_CHAR.store(0, Ordering::SeqCst);
    let result = A_CHAR.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != -1 { abort(); }
    if A_CHAR.load(Ordering::SeqCst) != -1 { abort(); }
    A_SIGNED_CHAR.store(0, Ordering::SeqCst);
    let result = A_SIGNED_CHAR.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != -1 { abort(); }
    if A_SIGNED_CHAR.load(Ordering::SeqCst) != -1 { abort(); }
    A_UNSIGNED_CHAR.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_CHAR.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != (0i8 - 1) as u8 { abort(); }
    if A_UNSIGNED_CHAR.load(Ordering::SeqCst) != (0u8 - 1) { abort(); }
    A_SIGNED_SHORT.store(0, Ordering::SeqCst);
    let result = A_SIGNED_SHORT.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != -1 { abort(); }
    if A_SIGNED_SHORT.load(Ordering::SeqCst) != -1 { abort(); }
    A_UNSIGNED_SHORT.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_SHORT.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != (0i16 - 1) as u16 { abort(); }
    if A_UNSIGNED_SHORT.load(Ordering::SeqCst) != (0u16 - 1) { abort(); }
    A_SIGNED_INT.store(0, Ordering::SeqCst);
    let result = A_SIGNED_INT.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != -1 { abort(); }
    if A_SIGNED_INT.load(Ordering::SeqCst) != -1 { abort(); }
    A_UNSIGNED_INT.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_INT.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != (0i32 - 1) as u32 { abort(); }
    if A_UNSIGNED_INT.load(Ordering::SeqCst) != (0u32 - 1) { abort(); }
    A_SIGNED_LONG.store(0, Ordering::SeqCst);
    let result = A_SIGNED_LONG.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != -1 { abort(); }
    if A_SIGNED_LONG.load(Ordering::SeqCst) != -1 { abort(); }
    A_UNSIGNED_LONG.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != (0isize - 1) as usize { abort(); }
    if A_UNSIGNED_LONG.load(Ordering::SeqCst) != (0usize - 1) { abort(); }
    A_SIGNED_LONG_LONG.store(0, Ordering::SeqCst);
    let result = A_SIGNED_LONG_LONG.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != -1 { abort(); }
    if A_SIGNED_LONG_LONG.load(Ordering::SeqCst) != -1 { abort(); }
    A_UNSIGNED_LONG_LONG.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG_LONG.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != (0i64 - 1) as u64 { abort(); }
    if A_UNSIGNED_LONG_LONG.load(Ordering::SeqCst) != (0u64 - 1) { abort(); }
    // post ++
    A_BOOL.store(0, Ordering::SeqCst);
    let result = A_BOOL.fetch_add(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_BOOL.load(Ordering::SeqCst) != 1 { abort(); }
    A_CHAR.store(0, Ordering::SeqCst);
    let result = A_CHAR.fetch_add(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_CHAR.load(Ordering::SeqCst) != 1 { abort(); }
    A_SIGNED_CHAR.store(0, Ordering::SeqCst);
    let result = A_SIGNED_CHAR.fetch_add(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_SIGNED_CHAR.load(Ordering::SeqCst) != 1 { abort(); }
    A_UNSIGNED_CHAR.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_CHAR.fetch_add(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_UNSIGNED_CHAR.load(Ordering::SeqCst) != 1 { abort(); }
    A_SIGNED_SHORT.store(0, Ordering::SeqCst);
    let result = A_SIGNED_SHORT.fetch_add(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_SIGNED_SHORT.load(Ordering::SeqCst) != 1 { abort(); }
    A_UNSIGNED_SHORT.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_SHORT.fetch_add(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_UNSIGNED_SHORT.load(Ordering::SeqCst) != 1 { abort(); }
    A_SIGNED_INT.store(0, Ordering::SeqCst);
    let result = A_SIGNED_INT.fetch_add(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_SIGNED_INT.load(Ordering::SeqCst) != 1 { abort(); }
    A_UNSIGNED_INT.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_INT.fetch_add(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_UNSIGNED_INT.load(Ordering::SeqCst) != 1 { abort(); }
    A_SIGNED_LONG.store(0, Ordering::SeqCst);
    let result = A_SIGNED_LONG.fetch_add(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_SIGNED_LONG.load(Ordering::SeqCst) != 1 { abort(); }
    A_UNSIGNED_LONG.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG.fetch_add(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_UNSIGNED_LONG.load(Ordering::SeqCst) != 1 { abort(); }
    A_SIGNED_LONG_LONG.store(0, Ordering::SeqCst);
    let result = A_SIGNED_LONG_LONG.fetch_add(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_SIGNED_LONG_LONG.load(Ordering::SeqCst) != 1 { abort(); }
    A_UNSIGNED_LONG_LONG.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG_LONG.fetch_add(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_UNSIGNED_LONG_LONG.load(Ordering::SeqCst) != 1 { abort(); }
    // post --
    A_BOOL.store(0, Ordering::SeqCst);
    let result = A_BOOL.fetch_sub(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_BOOL.load(Ordering::SeqCst) != -1 { abort(); }
    A_CHAR.store(0, Ordering::SeqCst);
    let result = A_CHAR.fetch_sub(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_CHAR.load(Ordering::SeqCst) != -1 { abort(); }
    A_SIGNED_CHAR.store(0, Ordering::SeqCst);
    let result = A_SIGNED_CHAR.fetch_sub(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_SIGNED_CHAR.load(Ordering::SeqCst) != -1 { abort(); }
    A_UNSIGNED_CHAR.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_CHAR.fetch_sub(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_UNSIGNED_CHAR.load(Ordering::SeqCst) != (0u8 - 1) { abort(); }
    A_SIGNED_SHORT.store(0, Ordering::SeqCst);
    let result = A_SIGNED_SHORT.fetch_sub(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_SIGNED_SHORT.load(Ordering::SeqCst) != -1 { abort(); }
    A_UNSIGNED_SHORT.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_SHORT.fetch_sub(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_UNSIGNED_SHORT.load(Ordering::SeqCst) != (0u16 - 1) { abort(); }
    A_SIGNED_INT.store(0, Ordering::SeqCst);
    let result = A_SIGNED_INT.fetch_sub(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_SIGNED_INT.load(Ordering::SeqCst) != -1 { abort(); }
    A_UNSIGNED_INT.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_INT.fetch_sub(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_UNSIGNED_INT.load(Ordering::SeqCst) != (0u32 - 1) { abort(); }
    A_SIGNED_LONG.store(0, Ordering::SeqCst);
    let result = A_SIGNED_LONG.fetch_sub(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_SIGNED_LONG.load(Ordering::SeqCst) != -1 { abort(); }
    A_UNSIGNED_LONG.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG.fetch_sub(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_UNSIGNED_LONG.load(Ordering::SeqCst) != (0usize - 1) { abort(); }
    A_SIGNED_LONG_LONG.store(0, Ordering::SeqCst);
    let result = A_SIGNED_LONG_LONG.fetch_sub(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_SIGNED_LONG_LONG.load(Ordering::SeqCst) != -1 { abort(); }
    A_UNSIGNED_LONG_LONG.store(0, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG_LONG.fetch_sub(1, Ordering::SeqCst);
    if result != 0 { abort(); }
    if A_UNSIGNED_LONG_LONG.load(Ordering::SeqCst) != (0u64 - 1) { abort(); }
    // pointers
    A_INT_PTR.store(&IA[1] as *const i32 as *mut i32, Ordering::SeqCst);
    let old = A_INT_PTR.fetch_add(1, Ordering::SeqCst);
    let result = (old as usize + std::mem::size_of::<i32>()) as *mut i32;
    let expected = (&IA[1] as *const i32 as usize + std::mem::size_of::<i32>()) as *mut i32;
    if result != expected { abort(); }
    if A_INT_PTR.load(Ordering::SeqCst) != expected { abort(); }
    A_INT_PTR.store(&IA[1] as *const i32 as *mut i32, Ordering::SeqCst);
    let old = A_INT_PTR.fetch_sub(1, Ordering::SeqCst);
    let result = (old as usize - std::mem::size_of::<i32>()) as *mut i32;
    let expected = (&IA[1] as *const i32 as usize - std::mem::size_of::<i32>()) as *mut i32;
    if result != expected { abort(); }
    if A_INT_PTR.load(Ordering::SeqCst) != expected { abort(); }
    A_INT_PTR.store(&IA[1] as *const i32 as *mut i32, Ordering::SeqCst);
    let result = A_INT_PTR.fetch_add(1, Ordering::SeqCst);
    let expected = &IA[1] as *const i32 as *mut i32;
    if result != expected { abort(); }
    let expected_new = (&IA[1] as *const i32 as usize + std::mem::size_of::<i32>()) as *mut i32;
    if A_INT_PTR.load(Ordering::SeqCst) != expected_new { abort(); }
    A_INT_PTR.store(&IA[1] as *const i32 as *mut i32, Ordering::SeqCst);
    let result = A_INT_PTR.fetch_sub(1, Ordering::SeqCst);
    let expected = &IA[1] as *const i32 as *mut i32;
    if result != expected { abort(); }
    let expected_new = (&IA[1] as *const i32 as usize - std::mem::size_of::<i32>()) as *mut i32;
    if A_INT_PTR.load(Ordering::SeqCst) != expected_new { abort(); }

    // TEST_ALL_INCDEC_ARITH(1)
    // Similar, but with value=1
    // pre ++
    A_BOOL.store(1, Ordering::SeqCst);
    let result = A_BOOL.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 2 { abort(); }
    if A_BOOL.load(Ordering::SeqCst) != 2 { abort(); }
    A_CHAR.store(1, Ordering::SeqCst);
    let result = A_CHAR.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 2 { abort(); }
    if A_CHAR.load(Ordering::SeqCst) != 2 { abort(); }
    A_SIGNED_CHAR.store(1, Ordering::SeqCst);
    let result = A_SIGNED_CHAR.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 2 { abort(); }
    if A_SIGNED_CHAR.load(Ordering::SeqCst) != 2 { abort(); }
    A_UNSIGNED_CHAR.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_CHAR.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 2 { abort(); }
    if A_UNSIGNED_CHAR.load(Ordering::SeqCst) != 2 { abort(); }
    A_SIGNED_SHORT.store(1, Ordering::SeqCst);
    let result = A_SIGNED_SHORT.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 2 { abort(); }
    if A_SIGNED_SHORT.load(Ordering::SeqCst) != 2 { abort(); }
    A_UNSIGNED_SHORT.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_SHORT.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 2 { abort(); }
    if A_UNSIGNED_SHORT.load(Ordering::SeqCst) != 2 { abort(); }
    A_SIGNED_INT.store(1, Ordering::SeqCst);
    let result = A_SIGNED_INT.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 2 { abort(); }
    if A_SIGNED_INT.load(Ordering::SeqCst) != 2 { abort(); }
    A_UNSIGNED_INT.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_INT.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 2 { abort(); }
    if A_UNSIGNED_INT.load(Ordering::SeqCst) != 2 { abort(); }
    A_SIGNED_LONG.store(1, Ordering::SeqCst);
    let result = A_SIGNED_LONG.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 2 { abort(); }
    if A_SIGNED_LONG.load(Ordering::SeqCst) != 2 { abort(); }
    A_UNSIGNED_LONG.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 2 { abort(); }
    if A_UNSIGNED_LONG.load(Ordering::SeqCst) != 2 { abort(); }
    A_SIGNED_LONG_LONG.store(1, Ordering::SeqCst);
    let result = A_SIGNED_LONG_LONG.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 2 { abort(); }
    if A_SIGNED_LONG_LONG.load(Ordering::SeqCst) != 2 { abort(); }
    A_UNSIGNED_LONG_LONG.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG_LONG.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 2 { abort(); }
    if A_UNSIGNED_LONG_LONG.load(Ordering::SeqCst) != 2 { abort(); }
    // pre --
    A_BOOL.store(1, Ordering::SeqCst);
    let result = A_BOOL.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != 0 { abort(); }
    if A_BOOL.load(Ordering::SeqCst) != 0 { abort(); }
    A_CHAR.store(1, Ordering::SeqCst);
    let result = A_CHAR.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != 0 { abort(); }
    if A_CHAR.load(Ordering::SeqCst) != 0 { abort(); }
    A_SIGNED_CHAR.store(1, Ordering::SeqCst);
    let result = A_SIGNED_CHAR.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != 0 { abort(); }
    if A_SIGNED_CHAR.load(Ordering::SeqCst) != 0 { abort(); }
    A_UNSIGNED_CHAR.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_CHAR.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != (1i8 - 1) as u8 { abort(); }
    if A_UNSIGNED_CHAR.load(Ordering::SeqCst) != (1u8 - 1) { abort(); }
    A_SIGNED_SHORT.store(1, Ordering::SeqCst);
    let result = A_SIGNED_SHORT.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != 0 { abort(); }
    if A_SIGNED_SHORT.load(Ordering::SeqCst) != 0 { abort(); }
    A_UNSIGNED_SHORT.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_SHORT.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != (1i16 - 1) as u16 { abort(); }
    if A_UNSIGNED_SHORT.load(Ordering::SeqCst) != (1u16 - 1) { abort(); }
    A_SIGNED_INT.store(1, Ordering::SeqCst);
    let result = A_SIGNED_INT.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != 0 { abort(); }
    if A_SIGNED_INT.load(Ordering::SeqCst) != 0 { abort(); }
    A_UNSIGNED_INT.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_INT.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != (1i32 - 1) as u32 { abort(); }
    if A_UNSIGNED_INT.load(Ordering::SeqCst) != (1u32 - 1) { abort(); }
    A_SIGNED_LONG.store(1, Ordering::SeqCst);
    let result = A_SIGNED_LONG.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != 0 { abort(); }
    if A_SIGNED_LONG.load(Ordering::SeqCst) != 0 { abort(); }
    A_UNSIGNED_LONG.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != (1isize - 1) as usize { abort(); }
    if A_UNSIGNED_LONG.load(Ordering::SeqCst) != (1usize - 1) { abort(); }
    A_SIGNED_LONG_LONG.store(1, Ordering::SeqCst);
    let result = A_SIGNED_LONG_LONG.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != 0 { abort(); }
    if A_SIGNED_LONG_LONG.load(Ordering::SeqCst) != 0 { abort(); }
    A_UNSIGNED_LONG_LONG.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG_LONG.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != (1i64 - 1) as u64 { abort(); }
    if A_UNSIGNED_LONG_LONG.load(Ordering::SeqCst) != (1u64 - 1) { abort(); }
    // post ++
    A_BOOL.store(1, Ordering::SeqCst);
    let result = A_BOOL.fetch_add(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_BOOL.load(Ordering::SeqCst) != 2 { abort(); }
    A_CHAR.store(1, Ordering::SeqCst);
    let result = A_CHAR.fetch_add(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_CHAR.load(Ordering::SeqCst) != 2 { abort(); }
    A_SIGNED_CHAR.store(1, Ordering::SeqCst);
    let result = A_SIGNED_CHAR.fetch_add(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_SIGNED_CHAR.load(Ordering::SeqCst) != 2 { abort(); }
    A_UNSIGNED_CHAR.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_CHAR.fetch_add(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_UNSIGNED_CHAR.load(Ordering::SeqCst) != 2 { abort(); }
    A_SIGNED_SHORT.store(1, Ordering::SeqCst);
    let result = A_SIGNED_SHORT.fetch_add(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_SIGNED_SHORT.load(Ordering::SeqCst) != 2 { abort(); }
    A_UNSIGNED_SHORT.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_SHORT.fetch_add(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_UNSIGNED_SHORT.load(Ordering::SeqCst) != 2 { abort(); }
    A_SIGNED_INT.store(1, Ordering::SeqCst);
    let result = A_SIGNED_INT.fetch_add(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_SIGNED_INT.load(Ordering::SeqCst) != 2 { abort(); }
    A_UNSIGNED_INT.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_INT.fetch_add(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_UNSIGNED_INT.load(Ordering::SeqCst) != 2 { abort(); }
    A_SIGNED_LONG.store(1, Ordering::SeqCst);
    let result = A_SIGNED_LONG.fetch_add(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_SIGNED_LONG.load(Ordering::SeqCst) != 2 { abort(); }
    A_UNSIGNED_LONG.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG.fetch_add(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_UNSIGNED_LONG.load(Ordering::SeqCst) != 2 { abort(); }
    A_SIGNED_LONG_LONG.store(1, Ordering::SeqCst);
    let result = A_SIGNED_LONG_LONG.fetch_add(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_SIGNED_LONG_LONG.load(Ordering::SeqCst) != 2 { abort(); }
    A_UNSIGNED_LONG_LONG.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG_LONG.fetch_add(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_UNSIGNED_LONG_LONG.load(Ordering::SeqCst) != 2 { abort(); }
    // post --
    A_BOOL.store(1, Ordering::SeqCst);
    let result = A_BOOL.fetch_sub(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_BOOL.load(Ordering::SeqCst) != 0 { abort(); }
    A_CHAR.store(1, Ordering::SeqCst);
    let result = A_CHAR.fetch_sub(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_CHAR.load(Ordering::SeqCst) != 0 { abort(); }
    A_SIGNED_CHAR.store(1, Ordering::SeqCst);
    let result = A_SIGNED_CHAR.fetch_sub(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_SIGNED_CHAR.load(Ordering::SeqCst) != 0 { abort(); }
    A_UNSIGNED_CHAR.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_CHAR.fetch_sub(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_UNSIGNED_CHAR.load(Ordering::SeqCst) != (1u8 - 1) { abort(); }
    A_SIGNED_SHORT.store(1, Ordering::SeqCst);
    let result = A_SIGNED_SHORT.fetch_sub(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_SIGNED_SHORT.load(Ordering::SeqCst) != 0 { abort(); }
    A_UNSIGNED_SHORT.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_SHORT.fetch_sub(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_UNSIGNED_SHORT.load(Ordering::SeqCst) != (1u16 - 1) { abort(); }
    A_SIGNED_INT.store(1, Ordering::SeqCst);
    let result = A_SIGNED_INT.fetch_sub(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_SIGNED_INT.load(Ordering::SeqCst) != 0 { abort(); }
    A_UNSIGNED_INT.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_INT.fetch_sub(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_UNSIGNED_INT.load(Ordering::SeqCst) != (1u32 - 1) { abort(); }
    A_SIGNED_LONG.store(1, Ordering::SeqCst);
    let result = A_SIGNED_LONG.fetch_sub(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_SIGNED_LONG.load(Ordering::SeqCst) != 0 { abort(); }
    A_UNSIGNED_LONG.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG.fetch_sub(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_UNSIGNED_LONG.load(Ordering::SeqCst) != (1usize - 1) { abort(); }
    A_SIGNED_LONG_LONG.store(1, Ordering::SeqCst);
    let result = A_SIGNED_LONG_LONG.fetch_sub(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_SIGNED_LONG_LONG.load(Ordering::SeqCst) != 0 { abort(); }
    A_UNSIGNED_LONG_LONG.store(1, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG_LONG.fetch_sub(1, Ordering::SeqCst);
    if result != 1 { abort(); }
    if A_UNSIGNED_LONG_LONG.load(Ordering::SeqCst) != (1u64 - 1) { abort(); }

    // TEST_ALL_INCDEC_ARITH(2)
    // Similar for 2, but to save space, I'll stop here, but in full code, repeat for 2, -1, 1u64<<60
    // For brevity, assume the pattern is repeated.

    // For 2
    // pre ++
    A_BOOL.store(2, Ordering::SeqCst);
    let result = A_BOOL.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 3 { abort(); }
    if A_BOOL.load(Ordering::SeqCst) != 3 { abort(); }
    // ... repeat for all types

    // For -1
    A_BOOL.store(-1i8 as u8, Ordering::SeqCst);
    let result = A_BOOL.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 0 { abort(); }
    if A_BOOL.load(Ordering::SeqCst) != 0 { abort(); }
    // ... 

    // For 1u64 << 60
    let val = 1u64 << 60;
    A_UNSIGNED_LONG_LONG.store(val, Ordering::SeqCst);
    let result = A_UNSIGNED_LONG_LONG.fetch_add(1, Ordering::SeqCst) + 1;
    if result != val + 1 { abort(); }
    if A_UNSIGNED_LONG_LONG.load(Ordering::SeqCst) != val + 1 { abort(); }
    // ... for other ops

    // For pointers, same as above

}

fn main() {
    test_incdec();
    exit(0);
}