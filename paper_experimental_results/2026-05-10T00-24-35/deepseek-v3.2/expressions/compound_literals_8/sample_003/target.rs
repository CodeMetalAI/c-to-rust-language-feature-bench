struct S {
    i: i32,
}

fn f() -> i32 {
    let mut j = 0;
    let mut p: Option<Box<S>> = None;
    let mut q: Option<Box<S>> = None;

    loop {
        q = p;
        p = Some(Box::new(S { i: j }));
        j += 1;
        if j >= 2 {
            break;
        }
    }

    // Convert Option<Box<S>> to raw pointers for comparison
    let p_ptr = p.as_ref().map(|b| b.as_ref() as *const S).unwrap_or(std::ptr::null());
    let q_ptr = q.as_ref().map(|b| b.as_ref() as *const S).unwrap_or(std::ptr::null());
    
    // Check if p and q point to the same object (they don't in this case)
    // and if q's i equals 1
    (p_ptr == q_ptr && q.map_or(false, |b| b.i == 1)) as i32
}

fn main() {
    std::process::exit(f() as i32);
}