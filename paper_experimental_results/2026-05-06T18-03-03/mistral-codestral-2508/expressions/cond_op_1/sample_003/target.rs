fn f(_p: &dyn std::any::Any) {}
fn f1(_p: &mut dyn std::any::Any) {}
fn f2(_p: &mut i32) {}
fn f4(_p: &i32) {}
fn f3(_p: &i32) {}

fn main() {
    let c_vp: &dyn std::any::Any = &();
    let vp: &mut dyn std::any::Any = &mut ();
    let c_ip: &i32 = &0;
    let v_ip: &mut i32 = &mut 0;
    let ip: &mut i32 = &mut 0;
    let c_cp: &str = "";

    f(if true { c_vp } else { c_ip });
    f(if true { c_ip } else { c_vp });

    f2(if true { v_ip } else { &mut 0 });
    f2(if true { &mut 0 } else { v_ip });

    f3(if true { c_ip } else { v_ip });
    f3(if true { v_ip } else { c_ip });

    f(if true { vp } else { c_cp });
    f(if true { c_cp } else { vp });

    f4(if true { ip } else { c_ip });
    f4(if true { c_ip } else { ip });

    f1(if true { vp } else { ip });
    f1(if true { ip } else { vp });
}