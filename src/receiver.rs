
use nes::{ErrorInfo,ErrorInfoTrait};

use std::collections::VecDeque;
use std::sync::mpsc;

use ::ThreadTrait;
use ::BrockenChannel;

pub struct Receiver<T:ThreadTrait,C> {
    pub receiver:mpsc::Receiver<C>,
    pub commands:VecDeque<C>,
    pub thread:T,
}

impl<T:ThreadTrait,C> Receiver<T,C> {
    pub fn new(receiver:mpsc::Receiver<C>,thread:T) -> Self {
        Receiver {
            receiver,
            commands:VecDeque::with_capacity(4),
            thread,
        }
    }

    pub fn recv_block(&mut self) -> Result<C,BrockenChannel<T>> {
        match self.commands.pop_back() {
            Some(command) => Ok(command),
            None => match self.receiver.recv() {
                Ok(command) => ok!(command),
                Err(_) => Err(self.error()),
            }
        }
    }

    pub fn recv(&mut self) -> Result<Option<C>,BrockenChannel<T>> {
        match self.commands.pop_back() {
            Some(command) => Ok(Some(command)),
            None => match self.receiver.try_recv() {
                Ok(command) => ok!(Some(command)),
                Err(mpsc::TryRecvError::Empty) => ok!(None),
                Err(_) => Err(self.error()),
            }
        }
    }

    pub fn error(&self) -> BrockenChannel<T> {
        BrockenChannel(self.thread.clone())
    }
}

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
                        Err(e) => break err!(Error::BrockenChannel, Box::new($receiver.error()) ),
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
                        Err(e) => break err!(Error::BrockenChannel, Box::new($receiver.error()) ),
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
