/*
 * cond_op_1
 */

const C_VP: Option<*const ()> = None;
let mut vp: Option<*mut ()> = None;
const C_IP: Option<*const i32> = None;
let mut v_ip: Option<&mut i32> = None;
let mut ip: Option<&mut i32> = None;
const C_CP: Option<*const char> = None;

fn f(p: Option<*const ()>) { let _ = p; }
fn f1(p: Option<*mut ()>) { let _ = p; }
fn f2(p: Option<&mut i32>) { let _ = p; }
fn f4(p: Option<*const i32>) { let _ = p; }
fn f3(p: Option<&mut i32>) { let _ = p; }

fn main() {
    f(if true { C_VP } else { C_IP });
    f(if true { C_IP } else { C_VP });

    f2(if true { v_ip } else { None });
    f2(if true { None } else { v_ip });

    f3(if true { C_IP } else { v_ip });
    f3(if true { v_ip } else { C_IP });

    f(if true { vp } else { C_CP });
    f(if true { C_CP } else { vp });

    f4(if true { ip } else { C_IP });
    f4(if true { C_IP } else { ip });

    f1(if true { vp } else { ip });
    f1(if true { ip } else { vp });
}