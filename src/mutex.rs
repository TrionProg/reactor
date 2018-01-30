
use std::sync::{MutexGuard,TryLockError};
use std::sync::Mutex as StdMutex;

use failure::Error;

use ::ThreadTrait;
use ::MutexPoisoned;

#[derive(Debug)]
pub struct Mutex<T> {
    tm:StdMutex<T>,
}

impl<T> Mutex<T> {
    pub fn new(t:T) -> Self {
        Mutex {
            tm:StdMutex::new(t)
        }
    }

    pub fn lock(&self) -> Result<MutexGuard<T>,Error> {
        match self.tm.lock() {
            Ok(tg) => ok!(tg),
            Err(_) => Err(Error::from(MutexPoisoned))
        }
    }

    pub fn try_lock(&self) -> Result<Option<MutexGuard<T>>,Error> {
        match self.tm.try_lock() {
            Ok(tg) => ok!(Some(tg)),
            Err(TryLockError::WouldBlock) => ok!(None),
            Err(TryLockError::Poisoned(_)) => Err(Error::from(MutexPoisoned))
        }
    }
}


#[macro_export]
macro_rules! mutex_lock {
    ( $mutex:expr => $var:ident) => {
        let mut guard=$mutex.lock()?;

        let $var=&mut guard;
    };
}
