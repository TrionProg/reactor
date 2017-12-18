
use nes::{ErrorInfo,ErrorInfoTrait};

use std::collections::VecDeque;
use std::sync::mpsc;

use ::ThreadTrait;
use ::ReactorError;

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

    pub fn recv(&mut self) -> Result<C,ReactorError<T>> {
        match self.commands.pop_back() {
            Some(command) => ok!(command),
            None => match self.receiver.recv() {
                Ok(command) => ok!(command),
                Err(_) => err!(ReactorError::BrockenChannel, self.thread.clone()),
            }
        }
    }

    pub fn try_recv(&mut self) -> Result<Option<C>,ReactorError<T>> {
        match self.commands.pop_back() {
            Some(command) => ok!(Some(command)),
            None => match self.receiver.try_recv() {
                Ok(command) => ok!(Some(command)),
                Err(mpsc::TryRecvError::Empty) => ok!(None),
                Err(_) => err!(ReactorError::BrockenChannel, self.thread.clone()),
            }
        }
    }
}

#[macro_export]
macro_rules! wait {
    ( $receiver:expr, $expect_ok:pat => $expect_ok_do:expr, $( $expect_err:pat => $expect_err_do:expr ),* ) => {
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
                            $expect_ok => break $expect_ok_do,
                            $(
                                $expect_err => $expect_err_do,
                            )*
                            _ => $receiver.commands.push_front(command),
                        },
                        Err(e) => return err!(ReactorError::BrockenChannel, $reactor.thread.clone()),
                    }
                },
                Some(command) => {
                    match command {
                        $expect_ok => $expect_ok_do,
                        _ => unreachable!(),
                    }
                }
            }
        }
    };
}
