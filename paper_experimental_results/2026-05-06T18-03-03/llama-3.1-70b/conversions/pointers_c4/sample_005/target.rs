fn f(x: i32) -> i32 { x }

fn main() {
    let p0: *const i32 = std::ptr::null();
    let v0: *const () = std::ptr::null();
    let p1: *const i32 = v0 as *const i32;

    let fp0: fn(i32) -> i32 = f;
    let fp1: fn(i32) -> i32 = fp0;
    let fp2: fn(i32) -> i32 = unsafe { std::mem::transmute(v0) };

    if p0 as usize != p1 as usize {
        return;
    }
    if p0 as usize != 0 {
        return;
    }

    if v0 as usize != 0 {
        return;
    }
    if v0 as usize != p0 as usize {
        return;
    }

    if fp0 as usize != fp1 as usize {
        return;
    }
    if fp1 as usize != fp2 as usize {
        return;
    }
    if fp0 as usize != 0 {
        return;
    }

    if p0 as usize != fp0 as usize {
        return;
    }
}