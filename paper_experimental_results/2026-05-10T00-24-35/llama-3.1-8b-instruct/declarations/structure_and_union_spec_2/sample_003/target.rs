fn main() {
    let s1 = Box::new(S { n: 0, d: vec![42.0; 8] });
    let s2 = Box::new(S { n: 0, d: vec![24.0; 5] });

    if s1.size_of() < s2.size_of() {
        panic!("size_of() failed");
    }

    if s1.offset_of_d()!= s1.size_of() {
        panic!("offset_of_d() failed");
    }

    if s1.d[0]!= 42.0 || s2.d[0]!= 24.0 {
        panic!("d[0] failed");
    }

    drop(s1);
    drop(s2);
}

struct S {
    n: i32,
    d: Vec<f64>,
}

impl S {
    fn size_of(&self) -> usize {
        self.d.capacity() + std::mem::size_of::<i32>()
    }

    fn offset_of_d(&self) -> usize {
        std::mem::size_of::<i32>()
    }
}