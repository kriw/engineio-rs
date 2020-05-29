use std::thread;
use std::time::{Duration, Instant};

use crossbeam::channel::{select, tick, unbounded, Receiver, Sender};

/// Bidirectional Channel
#[derive(Debug)]
pub struct BiChan<S, R> {
    pub tx: Sender<S>,
    pub rx: Receiver<R>,
}

impl<R, S> BiChan<R, S> {
    pub fn new() -> (BiChan<S, R>, BiChan<R, S>) {
        let (tx1, rx1) = unbounded();
        let (tx2, rx2) = unbounded();
        (Self::_new(tx1, rx2), Self::_new(tx2, rx1))
    }

    fn _new<T1, T2>(tx: Sender<T1>, rx: Receiver<T2>) -> BiChan<T1, T2> {
        BiChan { tx, rx }
    }
}

pub fn resettable_timeout(duration: Duration, resetter: Receiver<()>) -> Receiver<Instant> {
    let (tx, rx) = unbounded();
    thread::spawn(move || {
        let mut timeout = tick(duration);
        loop {
            select! {
                recv(resetter) -> _ => {
                    timeout = tick(duration);
                },
                recv(timeout) -> _ => {
                    unimplemented!()
                },
            };
        }
    });
    rx
}
