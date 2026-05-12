fn type_id<T>(x: &T) -> u32
where
    T: 'static,
{
    match x {
        _ if std::ptr::eq(x as *const _, &10 as *const i32) => 1,
        _ if x.is::<&[i32; 3]>() => 2,
        _ if x.is::<&str>() => 3,
        _ if x.is::<&[u8; 4]>() => 4,
        _ => 99,
    }
}

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    if type_id(&a)!= 1 {
        eprintln!("test failed!");
        std::process::exit(1);
    }
    if type_id(&a as *const [i32; 3])!= 2 {
        eprintln!("test failed!");
        std::process::exit(2);
    }

    if a[0]!= 10 {
        eprintln!("test failed!");
        std::process::exit(3);
    }
    if a[1]!= 20 {
        eprintln!("test failed!");
        std::process::exit(4);
    }
    if a[2]!= 30 {
        eprintln!("test failed!");
        std::process::exit(5);
    }

    if std::mem::size_of_val(&a)!= 3 * std::mem::size_of::<i32>() {
        eprintln!("test failed!");
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a)!= std::mem::align_of::<i32>() {
        eprintln!("test failed!");
        std::process::exit(7);
    }

    let s = "abc";
    if type_id(s.as_ptr())!= 3 {
        eprintln!("test failed!");
        std::process::exit(8);
    }
    if s.as_bytes()[0]!= b'a' {
        eprintln!("test failed!");
        std::process::exit(9);
    }
    if s.as_bytes()[1]!= b'b' {
        eprintln!("test failed!");
        std::process::exit(10);
    }
    if s.as_bytes()[2]!= b'c' {
        eprintln!("test failed!");
        std::process::exit(11);
    }
    if s.as_bytes()[3]!= b'\0' {
        eprintln!("test failed!");
        std::process::exit(12);
    }

    println!("all tests passed!");
    std::process::exit(0);
}