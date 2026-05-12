fn f(a: i32, t: i32, c: i32) -> i32 {
    a + t + c
}

fn main() {
    let a = 0;
    let c = 0;
    let t;
    
    let t_temp;
    let v = f(a, { t_temp = 3; t_temp + 2 }, c);
    t = t_temp;
    
    if v != 5 {
        std::process::exit(1);
    }
    
    let t1;
    let temp1;
    if { temp1 = 3; t + 2 } != 5 {
        std::process::exit(2);
    }
    t1 = temp1;
    
    let t2;
    let t3 = if 1 < 2 {
        let temp2;
        { temp2 = 0; temp2 + 2 }
    } else {
        let temp3;
        { temp3 = 1; temp3 + 2 }
    };
    if t3 != 2 {
        std::process::exit(3);
    }
    
    std::process::exit(0);
}