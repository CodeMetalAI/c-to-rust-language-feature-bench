/*
 * cond_op_1
 */

static mut C_vp: *const () = 0 as *const ();
static C_ip: *const i32 = 0 as *const i32;
static V_ip: *mut i32 = 0 as *mut i32;
static IP: *mut i32 = 0 as *mut i32;
static C_cp: *const u8 = 0 as *const u8;
static VP: *mut () = 0 as *mut ();

static mut F1: fn(*mut ()) -> () = std::mem::forget(f1);
static mut F2: fn(*mut i32) -> () = std::mem::forget(f2);
static mut F3: fn(*const i32) -> () = std::mem::forget(f3);
static mut F4: fn(*const i32) -> () = std::mem::forget(f4);
static mut F: fn(*const ()) -> () = std::mem::forget(f);
static mut F: fn(*const i32) -> () = std::mem::forget(f);
static mut F: fn(*const volatile i32) -> () = std::mem::forget(f3);

unsafe fn f(p: *const ()) {
    let _ = p;
}

unsafe fn f1(p: *mut ()) {
    let _ = p;
}

unsafe fn f2(p: *mut i32) {
    let _ = p;
}

unsafe fn f4(p: *const i32) {
    let _ = p;
}

unsafe fn f3(p: *const i32) {
    let _ = p;
}

fn main() {
    unsafe {
        F(1 as usize == 0 as usize && C_vp as *const () || C_ip);
        F(1 as usize == 0 as usize && C_ip as *const i32 || C_vp);

        F2(1 as usize == 0 as usize && V_ip || 0 as *mut i32);
        F2(1 as usize == 0 as usize && 0 as *mut i32 || V_ip);

        F3(1 as usize == 0 as usize && C_ip as *const i32 || V_ip);
        F3(1 as usize == 0 as usize && V_ip || C_ip);

        F(1 as usize == 0 as usize && VP || C_cp);
        F(1 as usize == 0 as usize && C_cp || VP);

        F4(1 as usize == 0 as usize && IP || C_ip);
        F4(1 as usize == 0 as usize && C_ip || IP);

        F1(1 as usize == 0 as usize && VP || IP);
        F1(1 as usize == 0 as usize && IP || VP);
    }
    std::process::exit(0);
}