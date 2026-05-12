static mut C_VP: Option<*const ()> = None;
static mut VP: Option<*const ()> = None;
static mut C_IP: Option<*const i32> = None;
static mut V_IP: Option<*const i32> = None;
static mut IP: Option<*const i32> = None;
static mut C_CP: Option<*const u8> = None;

fn f<T>(_p: Option<T>) {}
fn f1<T>(_p: Option<T>) {}
fn f2<T>(_p: Option<T>) {}
fn f3<T>(_p: Option<T>) {}
fn f4<T>(_p: Option<T>) {}

fn main() {
    let c_vp: Option<&()> = None;
    let vp: Option<&()> = None;
    let c_ip: Option<&i32> = None;
    let v_ip: Option<&i32> = None;
    let ip: Option<&i32> = None;
    let c_cp: Option<&u8> = None;

    f(if true { c_vp } else { c_ip.map(|_| &()) });
    f(if true { c_ip.map(|_| &()) } else { c_vp });

    f2(if true { v_ip } else { None });
    f2(if true { None } else { v_ip });

    f3(if true { c_ip } else { v_ip });
    f3(if true { v_ip } else { c_ip });

    f(if true { vp } else { c_cp.map(|_| &()) });
    f(if true { c_cp.map(|_| &()) } else { vp });

    f4(if true { ip } else { c_ip });
    f4(if true { c_ip } else { ip });

    f1(if true { vp } else { ip.map(|_| &()) });
    f1(if true { ip.map(|_| &()) } else { vp });
}