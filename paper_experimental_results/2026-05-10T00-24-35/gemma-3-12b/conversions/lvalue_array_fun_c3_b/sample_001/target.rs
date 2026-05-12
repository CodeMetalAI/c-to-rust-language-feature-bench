fn main() {
    let a = [10, 20, 30];

    if type_id(&a) != 2 {
        std::process::exit(1);
    }

    if type_id(&(&a[0])) != 1 {
        std::process::exit(2);
    }

    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    if a.len() * std::mem::size_of::<i32>() != std::mem::size_of::<[i32; 3]>() {
        std::process::exit(6);
    }
    if std::mem::align_of::<[i32; 3]>() != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s = "abc";
    if type_id(s) != 3 {
        std::process::exit(8);
    }
    if s.chars().nth(0).unwrap() != 'a' {
        std::process::exit(9);
    }
    if s.chars().nth(1).unwrap() != 'b' {
        std::process::exit(10);
    }
    if s.chars().nth(2).unwrap() != 'c' {
        std::process::exit(11);
    }
    if s.chars().nth(3).unwrap() != '\0' {
        std::process::exit(12);
    }

    std::process::exit(0);
}

#[allow(dead_code)]
fn type_id(x: &dyn std::any::Any) -> i32 {
    if let Some(_ptr) = x.downcast_ref::<&[i32]>(){
        2
    } else if let Some(_ptr) = x.downcast_ref::<*const i32>(){
        1
    } else if let Some(_ptr) = x.downcast_ref::<&str>(){
        3
    } else {
        99
    }
}