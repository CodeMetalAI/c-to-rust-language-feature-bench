/*
 * cond_op_1
 */

static C_VP: *const std::ffi::c_void = std::ptr::null();
static VP: *mut std::ffi::c_void = std::ptr::null_mut();
static C_IP: *const i32 = std::ptr::null();
static V_IP: *mut i32 = std::ptr::null_mut();
static IP: *mut i32 = std::ptr::null_mut();
static C_CP: *const u8 = std::ptr::null();

fn f(_: *const std::ffi::c_void) {}
fn f1(_: *mut std::ffi::c_void) {}
fn f2(_: *mut i32) {}
fn f3(_: *const i32) {}
fn f4(_: *const i32) {}

fn main() {
    // Using as _ to cast to the appropriate type
    f(if true { C_VP } else { C_IP as _ });
    f(if true { C_IP as _ } else { C_VP });

    f2(if true { V_IP } else { std::ptr::null_mut() });
    f2(if true { std::ptr::null_mut() } else { V_IP });

    f3(if true { C_IP } else { V_IP as _ });
    f3(if true { V_IP as _ } else { C_IP });

    f(if true { VP } else { C_CP as _ });
    f(if true { C_CP as _ } else { VP });

    f4(if true { IP as _ } else { C_IP });
    f4(if true { C_IP } else { IP as _ });

    f1(if true { VP } else { IP as _ });
    f1(if true { IP as _ } else { VP });
}