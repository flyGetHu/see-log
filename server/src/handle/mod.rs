use crate::handle::catcher_handle::CatcherHandle;
use salvo::Catcher;

pub mod catcher_handle;

pub fn inti_catcher() -> Vec<Box<dyn Catcher>> {
    vec![Box::new(CatcherHandle)]
}
