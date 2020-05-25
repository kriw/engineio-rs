use std::sync::mpsc::{channel, Receiver, Sender};

/// Bidirectional Channel
#[derive(Debug)]
pub struct BiChan<S, R> {
    tx: Sender<S>,
    rx: Receiver<R>,
}

impl<R, S> BiChan<R, S> {
    pub fn new() -> (BiChan<S, R>, BiChan<R, S>) {
        let (tx1, rx1) = channel();
        let (tx2, rx2) = channel();
        (Self::_new(tx1, rx2), Self::_new(tx2, rx1))
    }

    fn _new<T1, T2>(tx: Sender<T1>, rx: Receiver<T2>) -> BiChan<T1, T2> {
        BiChan { tx, rx }
    }
}
