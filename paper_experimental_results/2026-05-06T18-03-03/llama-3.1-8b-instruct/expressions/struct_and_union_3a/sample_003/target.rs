#[derive(Debug)]
struct U {
    alltypes: i32,
    type_: i32,
    intnode: i32,
    doublenode: f64,
}

fn main() {
    let mut u = U {
        alltypes: 0,
        type_: 0,
        intnode: 0,
        doublenode: 0.0,
    };

    u.type_ = 1;
    u.doublenode = 3.14;

    if u.alltypes!= 1 {
        println!("Error: u.alltypes!= 1");
        return;
    }
    if u.type_!= 1 {
        println!("Error: u.type_!= 1");
        return;
    }
    if u.type_!= 1 {
        println!("Error: u.type_!= 1");
        return;
    }
    if u.doublenode!= 3.14 {
        println!("Error: u.doublenode!= 3.14");
        return;
    }

    println!("All checks passed");
}