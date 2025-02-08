use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::mpsc::error::TryRecvError;

pub struct EatSpit<T> {
    rx: Receiver<T>,
    val: Option<T>,
    tx: Sender<T>,
}

impl<T> EatSpit<T> {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        EatSpit {
            rx,
            tx,
            val: None
        }
    }
    
    pub fn val(&mut self) -> &T {
        self.val.as_mut().unwrap()
    }

    pub fn val_mut(&mut self) -> &mut T {
        self.val.as_mut().unwrap()
    }
    
    
    pub fn mouth(&mut self) -> Sender<T> {
       self.tx.clone() 
    }
    
    pub fn available(&mut self) -> bool {
        if self.val.is_some() {
            return true;
        }
        if let Ok(x) = self.rx.try_recv() {
            self.val = Some(x);
            return true;
        } 
        
        false
    }
}
