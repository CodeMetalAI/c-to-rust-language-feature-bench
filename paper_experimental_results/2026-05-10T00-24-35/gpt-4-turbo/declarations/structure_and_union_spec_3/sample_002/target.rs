use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

struct S {
    i: i32,
    a: [i32],
}

fn main() {
    unsafe {
        let layout = Layout::new::<i32>().extend(Layout::array::<i32>(4).unwrap()).unwrap().0.pad_to_align();
        let p = alloc(layout) as *mut S;

        if p.is_null() {
            std::process::exit(3);
        }

        (*p).i = 7;
        (*p).a.as_mut_ptr().add(0).write(11);
        (*p).a.as_mut_ptr().add(3).write(22);

        if (*p).i != 7 || (*p).a.as_ptr().add(0).read() != 11 || (*p).a.as_ptr().add(3).read() != 22 {
            dealloc(p as *mut u8, layout);
            std::process::exit(4);
        }

        dealloc(p as *mut u8, layout);
    }
}