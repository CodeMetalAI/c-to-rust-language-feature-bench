fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 1;
    let px: *const i32 = &x;
    let pf: fn(i32) -> i32 = f;

    let n1: *const i32 = std::ptr::null();
    let v0: *const () = std::ptr::null();
    let n2: *const i32 = v0 as *const i32;

    if n1!= std::ptr::null() {
        println!("n1 is not null");
        return;
    }
    if n2!= std::ptr::null() {
        println!("n2 is not null");
        return;
    }
    if v0!= std::ptr::null() {
        println!("v0 is not null");
        return;
    }

    if n1 == px {
        println!("n1 is equal to px");
    }
    if n2 == px {
        println!("n2 is equal to px");
    }

    let nf1: fn(i32) -> i32 = std::ptr::null();
    let vf0: *const () = std::ptr::null();
    let nf2: fn(i32) -> i32 = vf0 as fn(i32) -> i32;

    if nf1!= std::ptr::null() {
        println!("nf1 is not null");
        return;
    }
    if nf2!= std::ptr::null() {
        println!("nf2 is not null");
        return;
    }
    if nf1 == pf {
        println!("nf1 is equal to pf");
    }
    if nf2 == pf {
        println!("nf2 is equal to pf");
    }

    let vx = px;
    let vf = std::mem::transmute(pf);

    if vx == std::ptr::null() {
        println!("vx is null");
    }
    if vf == std::ptr::null() {
        println!("vf is null");
    }
    if vx == v0 {
        println!("vx is equal to v0");
    }
    if vf == vf0 {
        println!("vf is equal to vf0");
    }

    println!("All comparisons passed");
}