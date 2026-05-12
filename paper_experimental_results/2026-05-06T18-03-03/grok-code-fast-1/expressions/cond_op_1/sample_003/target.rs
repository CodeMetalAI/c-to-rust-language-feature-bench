fn main() {
    let c_vp: usize = 0;
    let vp: usize = 0;
    let c_ip: usize = 0;
    let v_ip: usize = 0;
    let ip: usize = 0;
    let c_cp: usize = 0;

    fn f(_p: usize) {}
    fn f1(_p: usize) {}
    fn f2(_p: usize) {}
    fn f4(_p: usize) {}
    fn f3(_p: usize) {}

    f(if true { c_vp } else { c_ip });
    f(if true { c_ip } else { c_vp });

    f2(if true { v_ip } else { 0 });
    f2(if true { 0 } else { v_ip });

    f3(if true { c_ip } else { v_ip });
    f3(if true { v_ip } else { c_ip });

    f(if true { vp } else { c_cp });
    f(if true { c_cp } else { vp });

    f4(if true { ip } else { c_ip });
    f4(if true { c_ip } else { ip });

    f1(if true { vp } else { ip });
    f1(if true { ip } else { vp });
}