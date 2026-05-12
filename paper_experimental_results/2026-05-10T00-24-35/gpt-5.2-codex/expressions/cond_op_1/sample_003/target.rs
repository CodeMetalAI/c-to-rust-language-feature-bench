static C_VP: usize = 0;
static VP: usize = 0;
static C_IP: usize = 0;
static V_IP: usize = 0;
static IP: usize = 0;
static C_CP: usize = 0;

fn f(p: usize) {
    let _ = p;
}
fn f1(p: usize) {
    let _ = p;
}
fn f2(p: usize) {
    let _ = p;
}
fn f4(p: usize) {
    let _ = p;
}
fn f3(p: usize) {
    let _ = p;
}

fn main() {
    f(if true { C_VP } else { C_IP });
    f(if true { C_IP } else { C_VP });

    f2(if true { V_IP } else { 0 });
    f2(if true { 0 } else { V_IP });

    f3(if true { C_IP } else { V_IP });
    f3(if true { V_IP } else { C_IP });

    f(if true { VP } else { C_CP });
    f(if true { C_CP } else { VP });

    f4(if true { IP } else { C_IP });
    f4(if true { C_IP } else { IP });

    f1(if true { VP } else { IP });
    f1(if true { IP } else { VP });
}