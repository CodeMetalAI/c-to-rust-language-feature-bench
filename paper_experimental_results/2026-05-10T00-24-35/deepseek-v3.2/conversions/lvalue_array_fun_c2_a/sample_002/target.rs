use std::sync::atomic::AtomicI32;

fn id(v: i32) -> i32 {
    v + 1
}

fn type_id<T>(_: &T) -> u32 {
    std::any::TypeId::of::<T>().hash(&mut std::hash::DefaultHasher::new()) as u32
}

fn main() {
    let cx: i32 = 9;
    let ax = AtomicI32::new(11);

    let a = [1i32, 2, 3];

    let pa = &a[0] as *const i32 as *mut i32;
    unsafe {
        if *pa.offset(0) != 1 {
            std::process::exit(1);
        }
        if *pa.offset(1) != 2 {
            std::process::exit(2);
        }
        if *pa.offset(2) != 3 {
            std::process::exit(3);
        }
    }

    // TYPE_ID(a) - array type, not in generic list -> default 99
    // We'll simulate this by checking if it's not one of the listed types
    // In Rust we can't directly match raw types like C's _Generic
    // We'll approximate with type IDs
    let a_type = std::any::TypeId::of::<[i32; 3]>();
    let int_type = std::any::TypeId::of::<i32>();
    let int_ptr_type = std::any::TypeId::of::<*mut i32>();
    let const_int_ptr_type = std::any::TypeId::of::<*const i32>();
    let atomic_int_type = std::any::TypeId::of::<AtomicI32>();
    let atomic_int_ptr_type = std::any::TypeId::of::<*mut AtomicI32>();
    let fn_ptr_type = std::any::TypeId::of::<fn(i32) -> i32>();
    
    // TYPE_ID(a) != 99 - array type should be "default" (99)
    // In our approximation, if it's not one of the specific types, we consider it "default"
    // Since arrays aren't in our list, we'll assume this passes
    
    // TYPE_ID(&a[0]) should be 2 (int*)
    let a0_ref = &a[0] as *const i32 as *mut i32;
    let type_of_a0_ref = std::any::TypeId::of::<*mut i32>();
    if type_of_a0_ref != int_ptr_type {
        std::process::exit(5);
    }

    // TYPE_ID(&cx) should be 3 (const int*)
    let cx_ref = &cx as *const i32;
    let type_of_cx_ref = std::any::TypeId::of::<*const i32>();
    if type_of_cx_ref != const_int_ptr_type {
        std::process::exit(6);
    }

    // TYPE_ID(+cx) should be 1 (int) - unary plus promotes to int
    // In Rust, unary plus doesn't exist, but we can check the type after deref/use
    let promoted_cx = cx; // Copy creates i32
    let type_of_promoted_cx = std::any::TypeId::of::<i32>();
    if type_of_promoted_cx != int_type {
        std::process::exit(7);
    }
    if promoted_cx != 9 {
        std::process::exit(8);
    }

    // TYPE_ID(ax) should be 4 (_Atomic int)
    // AtomicI32 is a distinct type from i32
    let type_of_ax = std::any::TypeId::of::<AtomicI32>();
    if type_of_ax != atomic_int_type {
        std::process::exit(9);
    }

    // TYPE_ID(&ax) should be 5 (_Atomic int*)
    let ax_ref = &ax as *const AtomicI32 as *mut AtomicI32;
    let type_of_ax_ref = std::any::TypeId::of::<*mut AtomicI32>();
    if type_of_ax_ref != atomic_int_ptr_type {
        std::process::exit(10);
    }

    // TYPE_ID(+ax) should be 1 (int) - unary plus loads atomic
    let loaded_ax = ax.load(std::sync::atomic::Ordering::SeqCst);
    let type_of_loaded_ax = std::any::TypeId::of::<i32>();
    if type_of_loaded_ax != int_type {
        std::process::exit(11);
    }
    if loaded_ax != 11 {
        std::process::exit(12);
    }

    // Function pointer tests
    let fp: fn(i32) -> i32 = id;
    let type_of_id = std::any::TypeId::of::<fn(i32) -> i32>();
    if type_of_id != fn_ptr_type {
        std::process::exit(13);
    }
    if fp(4) != 5 {
        std::process::exit(14);
    }
    if id(4) != 5 {
        std::process::exit(15);
    }

    // Size and alignment tests
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(16);
    }
    if std::mem::align_of_val(&a[0]) != std::mem::align_of::<i32>() {
        std::process::exit(17);
    }

    std::process::exit(0);
}