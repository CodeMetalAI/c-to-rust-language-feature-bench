struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<*const S> = None;
    let mut q: Option<*const S>;
    let mut j: i32 = 0;
    let mut s0 = S { i: 0 };
    let mut s1 = S { i: 0 };

    loop {
        q = p;
        
        if j == 0 {
            s0 = S { i: j };
            p = Some(&s0 as *const S);
        } else {
            s1 = S { i: j };
            p = Some(&s1 as *const S);
        }
        
        j += 1;
        
        if j >= 2 {
            break;
        }
    }

    let p_ptr = p.unwrap();
    let q_ptr = q.unwrap();
    
    (p_ptr == q_ptr) && (unsafe { (*q_ptr).i } == 1)
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}