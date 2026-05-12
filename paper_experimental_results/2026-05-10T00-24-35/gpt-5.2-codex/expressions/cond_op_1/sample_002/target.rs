#[derive(Copy, Clone)]
struct ConstVoidPtr;
#[derive(Copy, Clone)]
struct VoidPtr;
#[derive(Copy, Clone)]
struct ConstIntPtr;
#[derive(Copy, Clone)]
struct VolatileIntPtr;
#[derive(Copy, Clone)]
struct IntPtr;
#[derive(Copy, Clone)]
struct ConstCharPtr;
#[derive(Copy, Clone)]
struct ConstVolatileIntPtr;

impl From<ConstIntPtr> for ConstVoidPtr {
    fn from(_: ConstIntPtr) -> Self {
        ConstVoidPtr
    }
}
impl From<ConstCharPtr> for ConstVoidPtr {
    fn from(_: ConstCharPtr) -> Self {
        ConstVoidPtr
    }
}
impl From<VoidPtr> for ConstVoidPtr {
    fn from(_: VoidPtr) -> Self {
        ConstVoidPtr
    }
}
impl From<IntPtr> for VoidPtr {
    fn from(_: IntPtr) -> Self {
        VoidPtr
    }
}
impl From<IntPtr> for ConstIntPtr {
    fn from(_: IntPtr) -> Self {
        ConstIntPtr
    }
}
impl From<ConstIntPtr> for ConstVolatileIntPtr {
    fn from(_: ConstIntPtr) -> Self {
        ConstVolatileIntPtr
    }
}
impl From<VolatileIntPtr> for ConstVolatileIntPtr {
    fn from(_: VolatileIntPtr) -> Self {
        ConstVolatileIntPtr
    }
}

const C_VP: Option<ConstVoidPtr> = None;
const VP: Option<VoidPtr> = None;
const C_IP: Option<ConstIntPtr> = None;
const V_IP: Option<VolatileIntPtr> = None;
const IP: Option<IntPtr> = None;
const C_CP: Option<ConstCharPtr> = None;

fn f(_p: Option<ConstVoidPtr>) {}
fn f1(_p: Option<VoidPtr>) {}
fn f2(_p: Option<VolatileIntPtr>) {}
fn f4(_p: Option<ConstIntPtr>) {}
fn f3(_p: Option<ConstVolatileIntPtr>) {}

fn map_opt<T, U: From<T>>(o: Option<T>) -> Option<U> {
    o.map(Into::into)
}

fn main() {
    f(if true { C_VP } else { map_opt(C_IP) });
    f(if true { map_opt(C_IP) } else { C_VP });

    f2(if true { V_IP } else { None });
    f2(if true { None } else { V_IP });

    f3(if true { map_opt(C_IP) } else { map_opt(V_IP) });
    f3(if true { map_opt(V_IP) } else { map_opt(C_IP) });

    f(if true { map_opt(VP) } else { map_opt(C_CP) });
    f(if true { map_opt(C_CP) } else { map_opt(VP) });

    f4(if true { map_opt(IP) } else { C_IP });
    f4(if true { C_IP } else { map_opt(IP) });

    f1(if true { VP } else { map_opt(IP) });
    f1(if true { map_opt(IP) } else { VP });
}