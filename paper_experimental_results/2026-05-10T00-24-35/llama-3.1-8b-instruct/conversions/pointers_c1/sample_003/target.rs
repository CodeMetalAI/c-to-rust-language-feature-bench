// pointers_c1.rs
// pointer_void_roundtrip.rs

#[derive(Debug)]
struct S {
    a: i32,
    b: f64,
}

fn main() {
    let xi: i32 = 42;
    let xd: f64 = 3.25;
    let mut xs = S {
        a: 7,
        b: 9.5,
    };

    let pi = &xi;
    let pd = &xd;
    let ps = &mut xs;

    let v1: *const i32 = pi as *const i32;
    let v2: *const f64 = pd as *const f64;
    let v3: *const S = ps as *const S;

    let pi2 = *v1 as *const i32;
    let pd2 = *v2 as *const f64;
    let ps2 = *v3 as *const S;

    if *pi2 != 42 {
        println!("Error: *pi2 != 42");
        return;
    }
    if *pd2 != 3.25 {
        println!("Error: *pd2 != 3.25");
        return;
    }
    if ps2.a != 7 {
        println!("Error: ps2.a != 7");
        return;
    }
    if ps2.b != 9.5 {
        println!("Error: ps2.b != 9.5");
        return;
    }

    let v1b = v1 as *const i32;
    let v2b = v2 as *const f64;
    let v3b = v3 as *const S;

    if v1b != v1 {
        println!("Error: v1b != v1");
        return;
    }
    if v2b != v2 {
        println!("Error: v2b != v2");
        return;
    }
    if v3b != v3 {
        println!("Error: v3b != v3");
        return;
    }

    println!("All assertions passed!");
    std::process::exit(0);
}