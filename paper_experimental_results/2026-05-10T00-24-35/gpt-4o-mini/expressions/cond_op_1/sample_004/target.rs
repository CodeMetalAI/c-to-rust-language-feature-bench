/*
 * cond_op_1
 */

const C_VP: *const () = std::ptr::null();
let mut vp: *mut () = std::ptr::null_mut();
const C_IP: *const i32 = std::ptr::null();
let mut v_ip: *mut i32 = std::ptr::null_mut();
let mut ip: *mut i32 = std::ptr::null_mut();
const C_CP: *const u8 = std::ptr::null();

fn f(_: *const ()) {}
fn f1(_: *mut ()) {}
fn f2(_: *mut i32) {}
fn f4(_: *const i32) {}
fn f3(_: *const *mut i32) {}

fn main() {
    f(if true { C_VP } else { C_IP });
    f(if true { C_IP } else { C_VP });

    f2(if true { v_ip } else { std::ptr::null_mut() });
    f2(if true { std::ptr::null_mut() } else { v_ip });

    f3(if true { C_IP } else { v_ip });
    f3(if true { v_ip } else { C_IP });

    f(if true { vp } else { C_CP });
    f(if true { C_CP } else { vp });

    f4(if true { ip } else { C_IP });
    f4(if true { C_IP } else { ip });

    f1(if true { vp } else { ip });
    f1(if true { ip } else { vp });

    std::process::exit(0);
}