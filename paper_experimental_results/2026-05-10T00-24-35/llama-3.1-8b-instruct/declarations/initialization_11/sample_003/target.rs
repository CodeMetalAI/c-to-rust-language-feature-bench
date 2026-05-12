struct W {
    a: [i32; 3],
    b: i32,
}

fn main() {
    let w = [
        W {
            a: [1, 0, 0],
            b: 0,
        },
        W {
            a: [2, 0, 0],
            b: 0,
        },
    ];

    if w.len()!= 2 {
        panic!("Expected 2 elements in array");
    }

    if w[0].a[0]!= 1 {
        panic!("Expected w[0].a[0] to be 1");
    }
    if w[0].a[1]!= 0 {
        panic!("Expected w[0].a[1] to be 0");
    }
    if w[0].a[2]!= 0 {
        panic!("Expected w[0].a[2] to be 0");
    }
    if w[0].b!= 0 {
        panic!("Expected w[0].b to be 0");
    }

    if w[1].a[0]!= 2 {
        panic!("Expected w[1].a[0] to be 2");
    }
    if w[1].a[1]!= 0 {
        panic!("Expected w[1].a[1] to be 0");
    }
    if w[1].a[2]!= 0 {
        panic!("Expected w[1].a[2] to be 0");
    }
    if w[1].b!= 0 {
        panic!("Expected w[1].b to be 0");
    }

    println!("All checks passed");
}