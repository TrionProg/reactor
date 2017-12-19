
#[macro_use]
extern crate nes;
use nes::{ErrorInfo};

#[macro_export]
pub mod receiver;
pub use receiver::Receiver;

#[macro_export]
pub mod sender;
pub use sender::Sender;


pub trait ThreadTrait:Clone + Copy + Eq + PartialEq + Display{}

pub struct BrockenChannel<T:ThreadTrait> (T);

use std::fmt::Display;
impl<T:ThreadTrait> std::fmt::Display for BrockenChannel<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Brocken Channel {}",self.0)
    }
}

impl<T:ThreadTrait> std::fmt::Debug for BrockenChannel<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Brocken Channel {}",self.0)
    }
}

pub fn create_channel<T:ThreadTrait,C>(thread:T) -> (Sender<T,C>, Receiver<T,C>) {
    let (sender, receiver) = std::sync::mpsc::channel();
    (Sender::new(sender, thread), Receiver::new(receiver, thread))
}
