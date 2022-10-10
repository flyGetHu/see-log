use salvo::Catcher;
use crate::handle::catcher_handle::Handle404;

pub mod catcher_handle;

pub fn inti_catcher() -> Vec<Box<dyn Catcher>> {
    vec![Box::new(Handle404)]
}