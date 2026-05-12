fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 3;
    let mut d = 4;

    if ((a < b) == (c < d))!= 1 {
        println!("Test 1 failed");
        return;
    }

    a = 2;
    b = 1;
    c = 4;
    d = 3;
    if ((a < b) == (c < d))!= 1 {
        println!("Test 2 failed");
        return;
    }

    a = 1;
    b = 2;
    c = 4;
    d = 3;
    if ((a < b) == (c < d))!= 0 {
        println!("Test 3 failed");
        return;
    }

    a = 2;
    b = 1;
    c = 3;
    d = 4;
    if ((a < b) == (c < d))!= 0 {
        println!("Test 4 failed");
        return;
    }

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    if ((a < b) == (c < d))!= 1 {
        println!("Test 5 failed");
        return;
    }

    println!("All tests passed");
}