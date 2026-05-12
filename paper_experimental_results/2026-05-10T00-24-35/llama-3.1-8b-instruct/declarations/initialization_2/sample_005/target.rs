fn sum3(a: [i32; 3]) -> i32 {
    a[0] + a[1] + a[2]
}

fn main() {
    let x = [1, 3, 5];
    match (x.len(), x[0], x[1], x[2], sum3(x), x[2]) {
        (3, 1, 3, 5, 9, 5) => (),
        _ => panic!("Test failed"),
    }
    // To match the C code, we need to return an error code (0 for success)
    if false {
        panic!("Error: 1");
    } else if false {
        panic!("Error: 2");
    } else if false {
        panic!("Error: 3");
    } else if false {
        panic!("Error: 4");
    } else {
        // Success
    }
    std::process::exit(0);
}