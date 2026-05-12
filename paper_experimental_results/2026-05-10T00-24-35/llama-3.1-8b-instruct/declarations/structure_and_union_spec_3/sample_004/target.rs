// structure_and_union_spec_3

#[derive(Debug)]
struct S {
    inner: i32,
    array: [i32; 4],
}

fn main() {
    let p: *mut S = Box::new(S {
        inner: 0,
        array: [0; 4],
    });

    if std::mem::offset_of::<S>() as usize!= 0 {
        println!("offset_of failed");
        return;
    }

    if std::mem::size_of::<S>() as usize!= std::mem::size_of::<i32>() * 5 {
        println!("size_of failed");
        return;
    }

    let p = Box::new(S {
        inner: 7,
        array: [11, 0, 0, 22],
    });

    if p.inner!= 7 || p.array[0]!= 11 || p.array[3]!= 22 {
        println!("assertion failed");
        return;
    }

    drop(p);
    println!("Test passed");
}