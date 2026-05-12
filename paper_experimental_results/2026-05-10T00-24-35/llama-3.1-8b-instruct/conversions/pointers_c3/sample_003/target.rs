fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x: i32 = 1;
    let px: *const i32 = &x;
    let pf: fn(i32) -> i32 = f;

    let n1: *const i32 = std::ptr::null();
    let v0: *const () = std::ptr::null();
    let n2: *const i32 = v0 as *const i32;

    let nf1: fn(i32) -> i32 = None;
    let vf0: *const () = std::ptr::null();
    let nf2: fn(i32) -> i32 = vf0 as Option<_>;

    if n1!= std::ptr::null() {
        println!("n1!= 0");
        return;
    }
    if n2!= std::ptr::null() {
        println!("n2!= 0");
        return;
    }
    if v0!= std::ptr::null() {
        println!("v0!= 0");
        return;
    }

    if n1 == px {
        println!("n1 == px");
        return;
    }
    if n2 == px {
        println!("n2 == px");
        return;
    }

    if nf1.is_some() {
        println!("nf1!= 0");
        return;
    }
    if nf2.is_some() {
        println!("nf2!= 0");
        return;
    }
    if nf1 == Some(pf) {
        println!("nf1 == pf");
        return;
    }
    if nf2 == Some(pf) {
        println!("nf2 == pf");
        return;
    }

    let vx: *const i32 = px;
    let vf: *const fn(i32) -> i32 = &pf as *const _;

    if vx == std::ptr::null() {
        println!("vx == 0");
        return;
    }
    if vf == std::ptr::null() {
        println!("vf == 0");
        return;
    }
    if vx == v0 {
        println!("vx == v0");
        return;
    }
    if vf == &pf as *const _ {
        println!("vf == vf0");
        return;
    }

    println!("0");
}