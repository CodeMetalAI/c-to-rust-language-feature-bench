fn main() {
    let a: [i32; 3] = [10, 20, 30];

    if a.len()!= 3 {
        panic!("Expected array length 3, but got {}", a.len());
    }
    if &a!= &a[..] {
        panic!("Expected array reference to be a reference to its first element");
    }

    if *a.iter().next().unwrap()!= 10 {
        panic!("Expected first element to be 10, but got {:?}", a);
    }
    if *a.iter().skip(1).next().unwrap()!= 20 {
        panic!("Expected second element to be 20, but got {:?}", a);
    }
    if *a.iter().skip(2).next().unwrap()!= 30 {
        panic!("Expected third element to be 30, but got {:?}", a);
    }

    if a.len() * std::mem::size_of::<i32>()!= 12 {
        panic!("Expected array size to be 12 bytes, but got {}", a.len() * std::mem::size_of::<i32>());
    }
    if std::mem::align_of::<i32>()!= std::mem::align_of::<[i32; 3]>() {
        panic!("Expected array alignment to be the same as i32 alignment, but got {} and {}", std::mem::align_of::<i32>(), std::mem::align_of::<[i32; 3]>());
    }

    let s = "abc";
    if s.len()!= 3 {
        panic!("Expected string length 3, but got {}", s.len());
    }
    if s.as_bytes()[0]!= b'a' {
        panic!("Expected first character to be 'a', but got '{:?}", s.as_bytes());
    }
    if s.as_bytes()[1]!= b'b' {
        panic!("Expected second character to be 'b', but got '{:?}", s.as_bytes());
    }
    if s.as_bytes()[2]!= b'c' {
        panic!("Expected third character to be 'c', but got '{:?}", s.as_bytes());
    }
    if s.as_bytes()[3]!= b'\0' {
        panic!("Expected fourth character to be '\0', but got '{:?}", s.as_bytes());
    }
}