
use std::sync::mpsc;
use failure::Error;

use ::ThreadTrait;

use super::BrockenChannel;
use super::Message;

pub struct Sender<T:ThreadTrait,S,C> {
    sender:mpsc::Sender< Message<T,S,C> >,
    to_thread:T,
    from_thread:T,
}

impl<T:ThreadTrait,S,C> Sender<T,S,C> {
    pub fn new(sender:mpsc::Sender< Message<T,S,C> >, to_thread:T) -> Self {
        Sender{
            sender,
            to_thread,
            from_thread:to_thread
        }
    }

    pub fn send_signal(&self, signal:S) -> Result<(), Error> {
        let message=Message::new_signal(self.from_thread,signal);

        match self.sender.send(message) {
            Ok(_) => Ok(()),
            Err(_) => bail!(BrockenChannel(self.to_thread.clone())),
        }
    }

    pub fn send_command(&self, command:C) -> Result<(), Error> {
        let message=Message::new_command(self.from_thread,command);

        match self.sender.send(message) {
            Ok(_) => Ok(()),
            Err(_) => bail!(BrockenChannel(self.to_thread.clone())),
        }
    }

    pub fn set_thread(&mut self, thread:T) {
        self.from_thread=thread;
    }
    //pub fn send_crash(found:T, thread:T)
}

impl<T:ThreadTrait,S,C> Clone for Sender<T,S,C> {
    fn clone(&self) -> Self {
        Sender {
            sender:self.sender.clone(),
            to_thread:self.to_thread.clone(),
            from_thread:self.from_thread.clone()
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
