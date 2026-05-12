use std::mem;
use std::alloc::{self, Layout};

struct S {
    i: i32,
    a: [i32],
}

fn main() {
    // Check offsets
    let i_offset = unsafe { &(*(0 as *const S)).i as *const _ as usize };
    if i_offset != 0 {
        std::process::exit(1);
    }

    let a_offset = unsafe { &(*(0 as *const S)).a as *const _ as usize };
    if a_offset != mem::size_of::<i32>() {
        std::process::exit(2);
    }

    let layout = Layout::new::<i32>()
        .extend(Layout::array::<i32>(4).unwrap())
        .unwrap()
        .0
        .pad_to_align();

    let ptr = unsafe { alloc::alloc(layout) as *mut i32 };
    if ptr.is_null() {
        std::process::exit(3);
    }

    unsafe {
        ptr.write(7); // Write to i
        ptr.add(1).write(11); // Write to a[0]
        ptr.add(4).write(22); // Write to a[3]
        if ptr.read() != 7 || ptr.add(1).read() != 11 || ptr.add(4).read() != 22 {
            std::process::exit(4);
        }
    }

    unsafe {
        alloc::dealloc(ptr as *mut u8, layout);
    }

    std::process::exit(0);
}