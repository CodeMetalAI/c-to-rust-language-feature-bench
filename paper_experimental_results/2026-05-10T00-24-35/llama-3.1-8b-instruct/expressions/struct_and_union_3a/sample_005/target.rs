struct UnionU {
    alltypes: i32,
    type_: i32,
    intnode: i32,
    doublenode: f64,
}

fn main() {
    let mut u = UnionU {
        alltypes: 0,
       ..Default::default()
    };

    u.type_ = 1;
    u.doublenode = 3.14;

    if u.alltypes!= 1 {
        println!("Error 1");
        return;
    }
    if u.type_!= 1 {
        println!("Error 2");
        return;
    }
    if u.type_!= 1 {
        println!("Error 3");
        return;
    }
    if u.doublenode!= 3.14 {
        println!("Error 4");
        return;
    }

    println!("Success");
}