use std;
use ::ThreadTrait;

pub mod sender;
pub use self::sender::Sender;

pub mod receiver;
pub use self::receiver::Receiver;

pub enum Message<T:ThreadTrait,S,C> {
    Signal(T,S),
    Command(T,C),
}

impl<T:ThreadTrait,S,C> Message<T,S,C> {
    fn new_signal(thread:T, signal:S) -> Self {
        Message::Signal(thread, signal)
    }

    fn new_command(thread:T, command:C) -> Self {
        Message::Command(thread, command)
    }
}

pub fn create_channel<T:ThreadTrait,S,C>(thread:T) -> (Sender<T,S,C>, Receiver<T,S,C>) {
    let (sender, receiver) = std::sync::mpsc::channel();
    (Sender::new(sender, thread), Receiver::new(receiver, thread))
}

#[derive(Fail, Debug)]
#[fail(display = "BrockenChannel: {}", _0)]
pub struct BrockenChannel<T:ThreadTrait> (T);
