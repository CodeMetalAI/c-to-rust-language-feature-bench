fn main() {
    let a = [10, 20, 30];

    if TYPE_ID(&a) != 2 {
        std::process::exit(1);
    }
    if TYPE_ID(&a) != 2 {
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

    let s = "abc".as_bytes();
    if TYPE_ID(s) != 3 {
        std::process::exit(8);
    }
    if s[0] != b'a' {
        std::process::exit(9);
    }
    if s[1] != b'b' {
        std::process::exit(10);
    }
    if s[2] != b'c' {
        std::process::exit(11);
    }
    if s[3] != 0 {
        std::process::exit(12);
    }

    std::process::exit(0);
}

#[allow(dead_code)]
fn TYPE_ID<T>(_: &T) -> i32 {
    let type_name = std::any::type_name::<T>();
    if type_name == "core::slice::Slice<i32>" {
        return 2;
    }
    if type_name == "core::slice::Slice<u8>" {
        return 3;
    }    
    99
}