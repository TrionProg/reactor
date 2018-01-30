
#[macro_use]
extern crate failure;

#[macro_use]
#[macro_export]
pub mod macros;

#[macro_export]
pub mod channel;
pub use channel::*;

#[macro_export]
pub mod mutex;
pub use mutex::Mutex;

use std::fmt::{Display,Debug};

pub trait ThreadTrait:Clone + Copy + Eq + Send + Sync + PartialEq + Display + Debug + 'static{}

#[derive(Fail, Debug)]
#[fail(display = "Mutex has been poisoned")]
pub struct MutexPoisoned;
