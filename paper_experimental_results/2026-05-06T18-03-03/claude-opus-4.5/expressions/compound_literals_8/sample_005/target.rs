fn f() -> bool {
    let mut storage: Option<Box<i32>> = None;
    let mut p: *const i32 = std::ptr::null();
    let mut q: *const i32;
    let mut j = 0;

    loop {
        q = p;
        // Simulate compound literal reusing same storage
        if storage.is_none() {
            storage = Some(Box::new(j));
        } else {
            *storage.as_mut().unwrap() = j;
        }
        p = storage.as_ref().unwrap().as_ref() as *const i32;
        j += 1;
        
        if j >= 2 {
            break;
        }
    }

    // Check p == q and *q == 1
    // After loop: both p and q point to same storage containing value 1
    (p == q) && (unsafe { *q } == 1)
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}