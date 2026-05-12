fn main() {
    struct S {
        _nested: i32,
        a: [i32; 4],
    };

    let p: *mut S = std::alloc::alloc(std::mem::size_of::<S>() + 4 * std::mem::size_of::<i32>());

    if std::mem::offset_of::<S>(_nested: i32)!= 0 {
        println!("offset of i is not 0");
        return;
    }

    if std::mem::offset_of::<S>(a: [i32; 4])!= std::mem::size_of::<S>() {
        println!("offset of a is not equal to size of S");
        return;
    }

    unsafe {
        *p = S {
            _nested: 7,
            a: [11, 0, 0, 22],
        };

        if (*p)._nested!= 7 || p.as_ref().unwrap().a[0]!= 11 || p.as_ref().unwrap().a[3]!= 22 {
            println!("values are not correct");
            return;
        }

        std::alloc::dealloc(p, std::alloc::Layout::new(std::mem::size_of::<S>() + 4 * std::mem::size_of::<i32>()));
    }
}