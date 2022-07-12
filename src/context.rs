use crate::{config::Persist, git::GitConfigClientType};

pub struct AppContext {
    pub git_config_client: Box<dyn GitConfigClientType>,
    pub config_client: Box<dyn Persist>,
}
