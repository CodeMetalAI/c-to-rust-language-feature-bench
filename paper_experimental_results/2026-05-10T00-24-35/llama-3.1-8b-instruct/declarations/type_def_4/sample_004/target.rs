// type_def_4.rs

enum HandlerKind {
    HandlerA,
    HandlerB,
}

struct Handler {
    kind: HandlerKind,
}

impl Handler {
    fn new(kind: HandlerKind) -> Self {
        Self { kind }
    }

    fn handler(&self) -> &'staticdyn Fn(i32) {
        match self.kind {
            HandlerKind::HandlerA => |x| {
                super::hits += x + 1;
            },
            HandlerKind::HandlerB => |x| {
                super::hits += x + 2;
            },
        }
    }
}

trait Signal {
    fn handle(&self, arg: i32);
}

struct SignalPlain;

impl Signal for SignalPlain {
    fn handle(&self, arg: i32) {
        super::signal(1, &Handler::new(HandlerKind::HandlerA));
        super::signal(1, &Handler::new(HandlerKind::HandlerB))(arg);
    }
}

struct SignalFv;

impl Signal for SignalFv {
    fn handle(&self, arg: i32) {
        let h = Handler::new(HandlerKind::HandlerB);
        super::signal(2, &h);
        h.handler()(arg);
    }
}

struct SignalPfv;

impl Signal for SignalPfv {
    fn handle(&self, arg: i32) {
        let h = Handler::new(HandlerKind::HandlerB);
        let r = super::signal(3, &h);
        r(arg);
    }
}

static mut HITS: i32 = 0;

fn signal(kind: i32, handler: &dyn Signal) {
    match kind & 1 {
        1 => {
            let h = Handler::new(HandlerKind::HandlerA);
            let r = h.handler();
            r(0);
        },
        0 => {
            let r = handler.handle(0);
        },
        _ => unreachable!(),
    }
}

fn call_signal_as_plain(kind: i32, handler: &dyn Signal, arg: i32) -> i32 {
    let r = signal(kind, handler);
    r.handle(arg);
    super::HITS
}

fn call_signal_as_fv(kind: i32, handler: &dyn Signal, arg: i32) -> i32 {
    let h = handler;
    let r = signal(kind, h);
    r.handle(arg);
    super::HITS
}

fn call_signal_as_pfv(kind: i32, handler: &dyn Signal, arg: i32) -> i32 {
    let r = signal(kind, handler);
    r.handle(arg);
    super::HITS
}

fn main() {
    super::HITS = 0;

    if call_signal_as_plain(1, || { }, 3) != 4 {
        std::process::exit(1);
    }

    super::HITS = 0;
    if call_signal_as_fv(2, || { }, 4) != 6 {
        std::process::exit(2);
    }

    super::HITS = 0;
    if call_signal_as_pfv(3, || { }, 5) != 6 {
        std::process::exit(3);
    }

    std::process::exit(0);
}