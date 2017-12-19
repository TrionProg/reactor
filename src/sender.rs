
use nes::{ErrorInfo,ErrorInfoTrait};

use std::sync::mpsc;

use ::ThreadTrait;
use ::BrockenChannel;

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

    pub fn send(&mut self, command:C) -> Result<(), BrockenChannel<T>> {
        match self.sender.send(command) {
            Ok(_) => Ok(()),
            Err(_) => Err(BrockenChannel(self.thread.clone())),
        }
    }

    //pub fn send_crash(found:T, thread:T) 
}

impl<T:ThreadTrait,C> Clone for Sender<T,C> {
    fn clone(&self) -> Self {
        Sender {
            sender:self.sender.clone(),
            thread:self.thread.clone()
        }
    }
}

#[macro_export]
macro_rules! try_send{
    [ $sender:expr, $message:expr ] => {
        try!($sender.send($message), Error::BrockenChannel)
        //$sender.send($message).unwrap()
    };
    [ $channel:expr, $message:expr, $error:ident ] => {
        try!($sender.send($message), $error::BrockenChannel)
    };
    [ $channel:expr, $message:expr, $error:path ] => {
        try!($sender.send($message), $error)
    };
    [ $channel:expr, $message:expr, $error:path , $( $arg:expr ),* ] => {
        try!($sender.send($message), $error, $( $arg, )*)
    };
}

#[macro_export]
macro_rules! send{
    [ $( $sender:expr , $message:expr ),* ] => {
        {
            let mut errors=Vec::new();

            $(
                match $sender.send($message) {
                    Ok(_) => {},
                    Err(e) => errors.push(e),
                }
            )*

            if errors.len()==0 {
                Ok(())
            }else{
                Err(errors)
            }
        }
    };
}

/*
[ $( $sender:expr <= $message:expr ),* ] => {
    {
        $( let $sender=&mut $sender; )

        let send=||{
            let mut errors=Vec::new();

            ($ match $sender.send($message) {
                Ok(_) => {},
                Err(e) => errors.push(create_err!(Error::BrockenChannel, e)),
            })

            errors
        };

        send()
    }
};
*/
