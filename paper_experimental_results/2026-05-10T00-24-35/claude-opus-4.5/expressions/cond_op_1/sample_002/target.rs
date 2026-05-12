static mut C_VP: *const () = std::ptr::null();
static mut VP: *mut () = std::ptr::null_mut();
static mut C_IP: *const i32 = std::ptr::null();
static mut V_IP: *mut i32 = std::ptr::null_mut();
static mut IP: *mut i32 = std::ptr::null_mut();
static mut C_CP: *const i8 = std::ptr::null();

fn f<T>(_p: Option<&T>) {}
fn f1<T>(_p: Option<&T>) {}
fn f2(_p: Option<&i32>) {}
fn f4(_p: Option<&i32>) {}
fn f3(_p: Option<&i32>) {}

fn main() {
    let c_vp: Option<&()> = None;
    let c_ip: Option<&i32> = None;
    let v_ip: Option<&i32> = None;
    let vp: Option<&()> = None;
    let ip: Option<&i32> = None;
    let c_cp: Option<&i8> = None;

    // f(1 ? c_vp : c_ip) - both become Option<&()> or similar generic
    f(if true { c_vp } else { None });
    f(if true { None } else { c_vp });

    // f2(1 ? v_ip : 0)
    f2(if true { v_ip } else { None });
    f2(if true { None } else { v_ip });

    // f3(1 ? c_ip : v_ip)
    f3(if true { c_ip } else { v_ip });
    f3(if true { v_ip } else { c_ip });

    // f(1 ? vp : c_cp)
    f(if true { vp } else { None });
    f(if true { None } else { vp });

    // f4(1 ? ip : c_ip)
    f4(if true { ip } else { c_ip });
    f4(if true { c_ip } else { ip });

    // f1(1 ? vp : ip)
    f1(if true { vp } else { None });
    f1(if true { None } else { vp });
}