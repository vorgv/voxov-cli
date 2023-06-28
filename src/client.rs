use reqwest::{
    blocking::{self, get},
    Error,
};

use crate::config::Config;

pub struct Client {
    config: Config,
    //TODO session: Session,
}

impl Client {
    pub fn new() -> Client {
        Client {
            config: Config::load(),
        }
    }

    /// Check connectivity.
    pub fn ping(&self) -> Result<String, Error> {
        get(&self.config.url)?.text()
    }

    ///TODO Authenticate.
    pub fn auth(&self) -> Result<String, Error> {
        blocking::Client::new()
            .post(&self.config.url)
            .header("type", "AuthSmsSendTo")
            .send()?
            .text()
    }
}
