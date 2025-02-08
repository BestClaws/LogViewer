use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::mpsc::error::TryRecvError;

pub struct EatSpit<T> {
    rx: Receiver<T>,
    val: T,
    tx: Sender<T>,
    gulped: bool,
}

impl<T> EatSpit<T> {
    pub fn new(default: T) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        EatSpit {
            rx,
            tx,
            val: default,
            gulped: false,
        }
    }
    
    pub fn mouth(&mut self) -> Sender<T> {
       self.tx.clone() 
    }
    
    pub fn spit(&mut self) -> &mut T {
        if let Ok(x) = self.rx.try_recv() {
            self.val = x;
        } 
        
        &mut self.val
    }
}
