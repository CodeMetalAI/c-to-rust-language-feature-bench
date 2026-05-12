fn main() {
    let p = [1e0f, 1e1f, 1e2f, 1e3f, 1e4f, 1e5f, 1e6f];

    if p[0]!= 1e0f {
        println!("Test 1 failed");
        return;
    }
    if p[1]!= 1e1f {
        println!("Test 2 failed");
        return;
    }
    if p[2]!= 1e2f {
        println!("Test 3 failed");
        return;
    }
    if p[3]!= 1e3f {
        println!("Test 4 failed");
        return;
    }
    if p[4]!= 1e4f {
        println!("Test 5 failed");
        return;
    }
    if p[5]!= 1e5f {
        println!("Test 6 failed");
        return;
    }
    if p[6]!= 1e6f {
        println!("Test 7 failed");
        return;
    }

    println!("All tests passed");
}