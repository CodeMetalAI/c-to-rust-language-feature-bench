struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<*const S> = None;
    let mut q: Option<*const S>;
    let mut j: i32 = 0;
    
    let mut s0 = S { i: 0 };
    let mut s1 = S { i: 0 };
    
    let literals = [&mut s0 as *mut S, &mut s1 as *mut S];
    let mut literal_index = 0;
    
    loop {
        q = p;
        
        unsafe {
            (*literals[literal_index]).i = j;
        }
        p = Some(literals[literal_index] as *const S);
        literal_index += 1;
        j += 1;
        
        if j >= 2 {
            break;
        }
    }
    
    if let (Some(p_ptr), Some(q_ptr)) = (p, q) {
        p_ptr == q_ptr && unsafe { (*q_ptr).i == 1 }
    } else {
        false
    }
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}