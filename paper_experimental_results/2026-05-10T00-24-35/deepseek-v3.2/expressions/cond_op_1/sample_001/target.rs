use std::ptr;

static C_VP: *const () = ptr::null();
static mut VP: *mut () = ptr::null_mut();
static C_IP: *const i32 = ptr::null();
static V_IP: *mut i32 = ptr::null_mut();
static mut IP: *mut i32 = ptr::null_mut();
static C_CP: *const i8 = ptr::null();

fn f(p: *const ()) {
    let _ = p;
}

fn f1(p: *mut ()) {
    let _ = p;
}

fn f2(p: *mut i32) {
    let _ = p;
}

fn f4(p: *const i32) {
    let _ = p;
}

fn f3(p: *const i32) {
    let _ = p;
}

fn main() {
    f(if true { C_VP } else { C_IP as *const () });
    f(if true { C_IP as *const () } else { C_VP });

    f2(if true { V_IP } else { ptr::null_mut() });
    f2(if true { ptr::null_mut() } else { V_IP });

    f3(if true { C_IP } else { V_IP as *const i32 });
    f3(if true { V_IP as *const i32 } else { C_IP });

    f(if true { unsafe { VP } } else { C_CP as *const () });
    f(if true { C_CP as *const () } else { unsafe { VP } });

    f4(if true { unsafe { IP } } else { C_IP });
    f4(if true { C_IP } else { unsafe { IP } });

    f1(if true { unsafe { VP } } else { unsafe { IP as *mut () } });
    f1(if true { unsafe { IP as *mut () } } else { unsafe { VP } });

    // Explicit return for consistency with C version
}