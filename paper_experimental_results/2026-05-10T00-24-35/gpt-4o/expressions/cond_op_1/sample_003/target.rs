/*
 * cond_op_1
 */

static C_VP: *const () = std::ptr::null();
static VP: *mut () = std::ptr::null_mut();
static C_IP: *const i32 = std::ptr::null();
static V_IP: *mut i32 = std::ptr::null_mut();
static IP: *mut i32 = std::ptr::null_mut();
static C_CP: *const u8 = std::ptr::null();

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
    f(if true { C_VP as *const () } else { C_IP as *const () });
    f(if true { C_IP as *const () } else { C_VP as *const () });

    f2(if true { V_IP } else { std::ptr::null_mut() });
    f2(if true { std::ptr::null_mut() } else { V_IP });

    f3(if true { C_IP as *const i32 } else { V_IP as *const i32 });
    f3(if true { V_IP as *const i32 } else { C_IP as *const i32 });

    f(if true { VP as *const () } else { C_CP as *const () });
    f(if true { C_CP as *const () } else { VP as *const () });

    f4(if true { IP as *const i32 } else { C_IP as *const i32 });
    f4(if true { C_IP as *const i32 } else { IP as *const i32 });

    f1(if true { VP } else { IP as *mut () });
    f1(if true { IP as *mut () } else { VP });
}