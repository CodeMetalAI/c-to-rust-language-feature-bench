fn sum3(a: [i32; 3]) -> i32 {
    a[0] + a[1] + a[2]
}

fn main() {
    let x = [1, 3, 5];

    if x.len()!= 3 {
        println!("Error: array length mismatch");
        return;
    }

    if x[0]!= 1 || x[1]!= 3 || x[2]!= 5 {
        println!("Error: array values mismatch");
        return;
    }

    if sum3(x)!= 9 {
        println!("Error: sum mismatch");
        return;
    }

    {
        let p = &x;
        if p[2]!= 5 {
            println!("Error: array value mismatch");
            return;
        }
    }

    println!("All tests passed");
}