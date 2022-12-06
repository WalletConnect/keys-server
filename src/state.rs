use crate::{config::Config, storage::Storage};

#[derive(Debug)]
pub struct State {
    pub config: Config,
    pub storage: Box<dyn Storage>,
}
