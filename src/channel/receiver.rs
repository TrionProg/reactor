
use std::collections::VecDeque;
use std::sync::mpsc;
use failure::Error;

use ::ThreadTrait;

use super::BrockenChannel;
use super::Message;

pub struct Receiver<T:ThreadTrait,S,C> {
    receiver:mpsc::Receiver< Message<T,S,C> >,
    deffered_commands:VecDeque< (T,C) >,
    thread:T,
}

/*
pub struct Signal<T:ThreadTrait,S> {
    pub thread:T,
    pub signal:S
}

pub struct Command<T:ThreadTrait,C> {
    pub thread:T,
    pub command:C
}
*/

impl<T:ThreadTrait,S,C> Receiver<T,S,C> {
    pub fn new(receiver:mpsc::Receiver< Message<T,S,C> >, thread:T) -> Self {
        Receiver {
            receiver,
            deffered_commands:VecDeque::with_capacity(4),
            thread,
        }
    }

    pub fn recv<F>(&mut self, filter:F) -> Result<Message<T,S,C>,Error> where
        F:Fn(&T,&C) -> bool
    {
        let mut delete=None;
        for (i,&(ref thread, ref command)) in self.deffered_commands.iter().enumerate() {
            if filter(thread,command) {
                delete=Some(i);
                break;
            }
        }

        match delete {
            Some(index) => {
                let (thread,command)=self.deffered_commands.remove(index).unwrap();
                return ok!( Message::new_command(thread, command) );
            },
            None => {
                loop {
                    match self.receiver.recv() {
                        Ok(message) => {
                            match message {
                                Message::Signal(..) => return ok!(message),
                                Message::Command(thread, command) => {
                                    if filter(&thread, &command) {
                                        return ok!( Message::new_command(thread, command) );
                                    }

                                    self.deffered_commands.push_back( (thread, command) );
                                }
                            }
                        },
                        Err(_) => bail!(self.error()),
                    }
                }
            }
        }
    }


    pub fn recv_nonblock<F>(&mut self, filter:F) -> Result<Option< Message<T,S,C> >,Error> where
        F:Fn(&T,&C) -> bool
    {
        let mut delete=None;
        for (i,&(ref thread, ref command)) in self.deffered_commands.iter().enumerate() {
            if filter(thread,command) {
                delete=Some(i);
                break;
            }
        }

        match delete {
            Some(index) => {
                let (thread,command)=self.deffered_commands.remove(index).unwrap();
                ok!(Some( Message::new_command(thread, command) ))
            },
            None => {
                match self.receiver.try_recv() {
                    Ok(message) => {
                        match message {
                            Message::Signal(..) => ok!(Some( message )),
                            Message::Command(thread, command) => {
                                if filter(&thread, &command) {
                                    ok!(Some( Message::new_command(thread, command) ))
                                }else{
                                    self.deffered_commands.push_back( (thread, command) );
                                    ok!(None)
                                }
                            }
                        }
                    },
                    Err(_) => bail!(self.error()),
                }
            }
        }
    }

/*
        //for i in 0..self.deffered_commands.len() {

        if self.deffered_commands.len()>0 {


        let
        match self.commands.pop_back() {
            Some(command) => Ok(command),
            None => match self.receiver.recv() {
                Ok((thread,command)) => ok!(command),
                Err(_) => bail!(self.error()),
            }
        }
    }
*/
    /*
    pub fn recv_noblock(&mut self) -> Result<Option<C>,Error> {
        match self.commands.pop_back() {
            Some(command) => Ok(Some(command)),
            None => match self.receiver.try_recv() {
                Ok(command) => ok!(Some(command)),
                Err(mpsc::TryRecvError::Empty) => ok!(None),
                Err(_) => bail!(self.error()),
            }
        }
    }
    */

    pub fn error(&self) -> BrockenChannel<T> {
        BrockenChannel(self.thread.clone())
    }
}

/*
#[macro_export]
macro_rules! try_recv{
    [ $receiver:expr ] => {
        try!($receiver.recv(), Error::BrockenChannel)
    };
    [ $channel:expr, $error:ident ] => {
        try!($receiver.recv(), $error::BrockenChannel)
    };
    [ $channel:expr, $error:path ] => {
        try!($receiver.recv(), $error)
    };
    [ $channel:expr, $error:path , $( $arg:expr ),* ] => {
        try!($receiver.recv(), $error, $( $arg, )*)
    };
}

#[macro_export]
macro_rules! try_recv_block{
    [ $receiver:expr ] => {
        try!($receiver.recv_block(), Error::BrockenChannel)
    };
    [ $channel:expr, $error:ident ] => {
        try!($receiver.recv_block(), $error::BrockenChannel)
    };
    [ $channel:expr, $error:path ] => {
        try!($receiver.recv_block(), $error)
    };
    [ $channel:expr, $error:path , $( $arg:expr ),* ] => {
        try!($receiver.recv_block(), $error, $( $arg, )*)
    };
}

*/

/*
let command_data=if $receiver.commands.len()>0 {
    let mut command_data=None;
    let mut new_commands=VecDeque::with_capacity($receiver.commands.len())

    while let Some(command)=$receiver.commands.pop_back() {
        let command=match $receiver.commands[i] {
            $expect_ok => command_data=Some($expect_ok_do),
            _ => new_commands.push_front(command),
        }

    $receiver.commands=new_commands;
}else{
    None
}
*/

/*
#[macro_export]
macro_rules! wait {
    [ $receiver:expr, $expect_ok:pat => $expect_ok_do:expr ] => {
        {
            use std::collections::VecDeque;
            let command_data=if $receiver.commands.len()>0 {
                let mut command_data=None;
                let mut new_commands=VecDeque::with_capacity($receiver.commands.len());

                while let Some(command)=$receiver.commands.pop_back() {
                    match command {
                        $expect_ok => command_data=Some($expect_ok_do),
                        _ => new_commands.push_front(command),
                    }
                }

                $receiver.commands=new_commands;
                command_data
            }else{
                None
            };

            match command_data {
                None => loop {
                    match $receiver.receiver.recv() {
                        Ok(command) => match command {
                            $expect_ok => break Ok($expect_ok_do),
                            _ => $receiver.commands.push_front(command),
                        },
                        Err(e) => Err(Error::from($receiver.error()))
                    }
                },
                Some(command_data) => Ok(command_data)
            }
        }
    };

    [ $receiver:expr, $expect_ok:pat => $expect_ok_do:expr, $( $expect_err:pat => $expect_err_do:expr ),* ] => {
        {
            let mut i=0;
            let command_exixts = loop {
                if i<$receiver.commands.len() {
                    match $receiver.commands[i] {
                        $expect_ok => break Some($receiver.commands.remove(i).unwrap()),
                        $(
                            $expect_err => $expect_err_do,
                        )*
                        _ => {},
                    }

                    i+=1;
                }else{
                    break None;
                }
            };

            match command_exixts {
                None => loop {
                    match $receiver.receiver.recv() {
                        Ok(command) => match command {
                            $expect_ok => break Ok($expect_ok_do),
                            $(
                                $expect_err => $expect_err_do,
                            )*
                            _ => $receiver.commands.push_front(command),
                        },
                        Err(e) => break Err(Error::from($receiver.error())),
                    }
                },
                Some(command) => {
                    match command {
                        $expect_ok => Ok($expect_ok_do),
                        _ => unreachable!(),
                    }
                }
            }
        }
    };
}
*/
