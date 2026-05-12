#[repr(C)]
struct s {
    i: int,
    a: [int; 4],
}

fn main() {
    if std::mem::offset_of!(s, i) != 0 {
        return 1;
    }

    if std::mem::offset_of!(s, a) != std::mem::size_of::<s>() {
        return 2;
    }

    let mut p = Box::new(s {
        i: 0,
        a: [0, 0, 0, 0],
    });

    if p.as_mut().i == 7 {
    } else if p.as_mut().i != 7 {
        return 4;
    }

    if p.as_mut().a[0] == 11 {
    } else if p.as_mut().a[0] != 11 {
        return 4;
    }

    if p.as_mut().a[3] == 22 {
    } else if p.as_mut().a[3] != 22 {
        return 4;
    }
    

    p.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;


    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        return 4;
    }
    
    drop(p);

    0
}