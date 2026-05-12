struct T1 {
    m: i32,
}

struct T2 {
    m: i32,
}

struct U12 {
    m: i32,  // Since both s1.m and s2.m share the same memory location
}

impl U12 {
    fn s1(&self) -> &T1 {
        // Safe reinterpretation since T1 just wraps an i32
        unsafe { &*(self as *const U12 as *const T1) }
    }
    fn s1_mut(&mut self) -> &mut T1 {
        unsafe { &mut *(self as *mut U12 as *mut T1) }
    }
    fn s2(&self) -> &T2 {
        unsafe { &*(self as *const U12 as *const T2) }
    }
    fn s2_mut(&mut self) -> &mut T2 {
        unsafe { &mut *(self as *mut U12 as *mut T2) }
    }
}