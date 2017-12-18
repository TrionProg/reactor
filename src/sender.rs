
use nes::{ErrorInfo,ErrorInfoTrait};

use std::sync::mpsc;

use ::ThreadTrait;
use ::ReactorError;

pub struct Sender<T:ThreadTrait,C> {
    pub sender:mpsc::Sender<C>,
    pub thread:T,
}

impl<T:ThreadTrait,C> Sender<T,C> {
    pub fn new(sender:mpsc::Sender<C>, thread:T) -> Self {
        Sender{
            sender,
            thread
        }
    }

    pub fn send(&mut self, command:C) -> Result<(), ReactorError<T>> {
        match self.send(command) {
            Ok(_) => ok!(),
            Err(_) => err!(ReactorError::BrockenChannel, self.thread.clone()),
        }
    }
}

impl<T:ThreadTrait,C> Clone for Sender<T,C> {
    fn clone(&self) -> Self {
        Sender {
            sender:self.sender.clone(),
            thread:self.thread.clone()
        }
    }
}
