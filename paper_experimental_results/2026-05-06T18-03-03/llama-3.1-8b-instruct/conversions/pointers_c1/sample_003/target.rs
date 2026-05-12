fn main() {
    let xi: i32 = 42;
    let xd: f64 = 3.25;
    let xs = S { a: 7, b: 9.5 };

    let pi = &xi;
    let pd = &xd;
    let ps = &xs;

    let v1: *const i32 = pi as *const i32;
    let v2: *const f64 = pd as *const f64;
    let v3: *const S = ps as *const S;

    let pi2: *const i32 = v1;
    let pd2: *const f64 = v2;
    let ps2: *const S = v3;

    if pi2!= pi {
        panic!("pi2!= pi");
    }
    if pd2!= pd {
        panic!("pd2!= pd");
    }
    if ps2!= ps {
        panic!("ps2!= ps");
    }

    if *pi2!= 42 {
        panic!("*pi2!= 42");
    }
    if *pd2!= 3.25 {
        panic!("*pd2!= 3.25");
    }
    if ps2.a!= 7 {
        panic!("ps2.a!= 7");
    }
    if ps2.b!= 9.5 {
        panic!("ps2.b!= 9.5");
    }

    let v1b: *const i32 = pi2 as *const i32;
    let v2b: *const f64 = pd2 as *const f64;
    let v3b: *const S = ps2 as *const S;

    if v1b!= v1 {
        panic!("v1b!= v1");
    }
    if v2b!= v2 {
        panic!("v2b!= v2");
    }
    if v3b!= v3 {
        panic!("v3b!= v3");
    }

    println!("Success!");
}

struct S {
    a: i32,
    b: f64,
}