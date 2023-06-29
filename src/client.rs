use reqwest::{
    blocking::{get, Client as ReqwestClient, RequestBuilder, Response},
    Error, IntoUrl,
};

use crate::config::Config;

pub struct Client {
    config: Config,
}

impl Client {
    /// Check connectivity.
    pub fn ping(&self) -> Result<String, Error> {
        get(&self.config.url)?.text()
    }

    ///TODO Authenticate.
    pub fn auth() {}

    pub fn auth_session_start() {}

    pub fn auth_session_refresh() {}

    pub fn auth_session_end() {}

    pub fn auth_sms_send_to(&self) -> Result<(String, String), Error> {
        let response = post(&self.config.url)
            .header("access", "todo")
            .header("refresh", "todo")
            .header("type", "AuthSmsSendTo")
            .send()?;
        let phone = get_header(&response, "phone");
        let message = get_header(&response, "message");
        Ok((phone, message))
    }
}

impl Default for Client {
    fn default() -> Self {
        Client {
            config: Config::load(),
        }
    }
}

fn post<U: IntoUrl>(url: U) -> RequestBuilder {
    ReqwestClient::new().post(url)
}

fn get_header(response: &Response, key: &str) -> String {
    response.headers()[key]
        .to_str()
        .unwrap_or_default()
        .to_string()
}
