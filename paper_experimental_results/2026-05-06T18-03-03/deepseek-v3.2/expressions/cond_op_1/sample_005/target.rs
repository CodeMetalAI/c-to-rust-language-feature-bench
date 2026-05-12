use std::ptr;

static C_VP: *const () = ptr::null();
static mut VP: *mut () = ptr::null_mut();
static C_IP: *const i32 = ptr::null();
static mut V_IP: *mut i32 = ptr::null_mut();
static mut IP: *mut i32 = ptr::null_mut();
static C_CP: *const i8 = ptr::null();

fn f(_p: *const ()) {}
fn f1(_p: *mut ()) {}
fn f2(_p: *mut i32) {}
fn f4(_p: *const i32) {}
fn f3(_p: *const i32) {}

fn main() {
    unsafe {
        f(if true { C_VP } else { C_IP as *const () });
        f(if true { C_IP as *const () } else { C_VP });

        f2(if true { V_IP } else { ptr::null_mut() });
        f2(if true { ptr::null_mut() } else { V_IP });

        f3(if true { C_IP } else { V_IP as *const i32 });
        f3(if true { V_IP as *const i32 } else { C_IP });

        f(if true { VP as *const () } else { C_CP as *const () });
        f(if true { C_CP as *const () } else { VP as *const () });

        f4(if true { IP as *const i32 } else { C_IP });
        f4(if true { C_IP } else { IP as *const i32 });

        f1(if true { VP } else { IP as *mut () });
        f1(if true { IP as *mut () } else { VP });
    }
}