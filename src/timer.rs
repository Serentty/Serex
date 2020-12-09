use core::time::Duration;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts::without_interrupts;

#[derive(Debug, Copy, Clone)]
pub struct Handler {
    function: fn(),
    time: Duration,    
    repeat: bool,
    countdown: Duration
}

impl Handler {
    pub const fn new(function: fn(), time: Duration, repeat: bool) -> Handler {
        Handler { function, time, repeat, countdown: time }
    }
}

type HandlerList = [Option<Handler>; 4]; // TODO: Switch this to a vector so that we can handle
                                          // more than 4 events.

struct Notifier {
    handlers: HandlerList,
    handler_count: usize,
    uptime: Duration
}

impl Notifier {
    fn new() -> Notifier {
        Notifier {
            handlers: HandlerList::default(),
            handler_count: 0,
            uptime: Duration::from_nanos(0)
        }
    }

    fn notify_all(&mut self) {
        for handler_option in self.handlers.iter_mut() {
            if let Some(handler) = handler_option {
                if handler.countdown == Duration::from_nanos(0) {
                    (handler.function)();
                    if handler.repeat {
                        handler.countdown = handler.time;
                    } else {
                        *handler_option = None;
                    }
                }
            }
        }
    }

    fn register_handler(&mut self, handler: Handler) -> Result<(), ()> {
        if self.handler_count < self.handlers.len() {
            self.handlers[self.handler_count] = Some(handler);
            Ok(())
        } else {
            Err(())
        }
    }

    fn advance(&mut self, delta: Duration) {
        self.uptime += delta;
        for handler in self.handlers.iter_mut() {
            if let Some(handler) = handler {
                if let Some(new_time) = handler.countdown.checked_sub(delta) {
                    handler.countdown = new_time;
                } else {
                    handler.countdown = Duration::from_nanos(0);
                }
            }
        }
        self.notify_all();
    }
}

lazy_static! {
    static ref NOTIFIER: Mutex<Notifier> = Mutex::new(Notifier::new());
}

pub fn register_handler(handler: Handler) -> Result<(), ()> {
    without_interrupts(|| {
        NOTIFIER.lock().register_handler(handler)
    })
}

pub fn advance(delta: Duration) {
    without_interrupts(|| {
        NOTIFIER.lock().advance(delta);
    });
}

pub fn uptime() -> Duration {
    NOTIFIER.lock().uptime
}