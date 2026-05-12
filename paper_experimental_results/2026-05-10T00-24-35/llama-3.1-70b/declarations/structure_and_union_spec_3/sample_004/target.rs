fn main() {
    let mut p: Box<[i32; 4]> = Box::new([0, 0, 0, 0]);

    if std::mem::size_of_val(&p[0])!= 0 {
        return 1;
    }

    if std::mem::size_of::<[i32; 4]>()!= std::mem::size_of_val(&p) {
        return 2;
    }

    unsafe {
        std::ptr::write(p.as_mut_ptr() as *mut i32, 7);
        std::ptr::write(p.as_mut_ptr().add(1), 11);
        std::ptr::write(p.as_mut_ptr().add(4), 22);
    }

    if unsafe { *p.as_ref().get_unchecked(0)!= 7 }
        || unsafe { *p.as_ref().get_unchecked(1)!= 11 }
        || unsafe { *p.as_ref().get_unchecked(4)!= 22 }
    {
        return 4;
    }
}