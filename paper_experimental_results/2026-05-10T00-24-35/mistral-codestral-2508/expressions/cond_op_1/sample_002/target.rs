fn f(_p: *const ()) {}
fn f1(_p: *mut ()) {}
fn f2(_p: *mut i32) {}
fn f3(_p: *const i32) {}
fn f4(_p: *const i32) {}

static C_VP: *const () = std::ptr::null();
static mut VP: *mut () = std::ptr::null_mut();
static C_IP: *const i32 = std::ptr::null();
static mut V_IP: *mut i32 = std::ptr::null_mut();
static mut IP: *mut i32 = std::ptr::null_mut();
static C_CP: *const u8 = std::ptr::null();

fn main() {
    unsafe {
        f(if 1 != 0 { C_VP } else { C_IP as *const () });
        f(if 1 != 0 { C_IP as *const () } else { C_VP });

        f2(if 1 != 0 { V_IP } else { std::ptr::null_mut() });
        f2(if 1 != 0 { std::ptr::null_mut() } else { V_IP });

        f3(if 1 != 0 { C_IP } else { V_IP as *const i32 });
        f3(if 1 != 0 { V_IP as *const i32 } else { C_IP });

        f(if 1 != 0 { VP as *const () } else { C_CP as *const () });
        f(if 1 != 0 { C_CP as *const () } else { VP as *const () });

        f4(if 1 != 0 { IP as *const i32 } else { C_IP });
        f4(if 1 != 0 { C_IP } else { IP as *const i32 });

        f1(if 1 != 0 { VP } else { IP as *mut () });
        f1(if 1 != 0 { IP as *mut () } else { VP });
    }
}