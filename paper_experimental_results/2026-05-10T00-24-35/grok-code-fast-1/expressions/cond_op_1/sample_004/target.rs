type CVp = *const ();
type Vp = *mut ();
type CIp = *const i32;
type VIp = *const i32;
type Ip = *mut i32;
type CCp = *const u8;

fn f(_p: *const ()) {}
fn f1(_p: *mut ()) {}
fn f2(_p: *const i32) {}
fn f4(_p: *const i32) {}
fn f3(_p: *const i32) {}

fn main() {
    let c_vp: CVp = std::ptr::null();
    let vp: Vp = std::ptr::null_mut();
    let c_ip: CIp = std::ptr::null();
    let v_ip: VIp = std::ptr::null();
    let ip: Ip = std::ptr::null_mut();
    let c_cp: CCp = std::ptr::null();

    f(if true { c_vp } else { c_ip as *const () });
    f(if true { c_ip as *const () } else { c_vp });

    f2(if true { v_ip } else { std::ptr::null() });
    f2(if true { std::ptr::null() } else { v_ip });

    f3(if true { c_ip } else { v_ip });
    f3(if true { v_ip } else { c_ip });

    f(if true { vp as *const () } else { c_cp as *const () });
    f(if true { c_cp as *const () } else { vp as *const () });

    f4(if true { ip as *const i32 } else { c_ip });
    f4(if true { c_ip } else { ip as *const i32 });

    f1(if true { vp } else { ip as *mut () });
    f1(if true { ip as *mut () } else { vp });
}