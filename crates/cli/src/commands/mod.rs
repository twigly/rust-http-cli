#[cfg(feature = "alias")]
pub(crate) mod alias;
pub(crate) mod args;
mod debug;
pub(crate) mod http;

use crate::{
    core::{Args, Result},
    shell::Shell,
};

type DonePtr = fn();

#[cfg(feature = "alias")]
const ALIAS_NAME_PREFIX: char = '@';

pub trait ArgsCommand<OD, O, E> {
    fn command(&mut self, os_dirs: &OD) -> Result<Box<dyn Command<OD, O, E>>>;
}

pub trait Command<OD, O, E> {
    fn execute(&self, shell: &mut Shell<OD, O, E>, args: &mut Args, done: DonePtr) -> Result<()>;
}
