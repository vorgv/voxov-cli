use std::process;

use reqwest::{
    blocking::{get, Client as ReqwestClient, RequestBuilder, Response},
    Error,
};

use crate::config::{Config, Session};

pub struct Client {
    config: Config,
    session: Session,
}

/// If response is error type, print error message and exit.
macro_rules! handle_error {
    ($response:expr) => {
        let t = get_header(&$response, "type");
        if t == "Error" {
            let e = get_header(&$response, "error");
            eprintln!("{}", e);
            process::exit(1);
        }
    };
}

impl Client {
    /// Check connectivity.
    pub fn ping(&self) -> Result<String, Error> {
        get(&self.config.url)?.text()
    }

    /// The http POST method.
    fn post(&self) -> RequestBuilder {
        ReqwestClient::new().post(&self.config.url)
    }

    ///TODO Authenticate.
    pub fn auth() {}

    pub fn auth_session_start(&self) -> Result<(String, String), Error> {
        let response = self.post().header("type", "AuthSessionStart").send()?;
        handle_error!(response);
        let access = get_header(&response, "access");
        let refresh = get_header(&response, "refresh");
        Ok((access, refresh))
    }

    pub fn auth_session_refresh(&self) -> Result<String, Error> {
        let response = self
            .post()
            .header("type", "AuthSessionRefresh")
            .header("refresh", &self.session.refresh)
            .send()?;
        handle_error!(response);
        let access = get_header(&response, "access");
        Ok(access)
    }

    pub fn auth_session_end() {}

    pub fn auth_sms_send_to(&self) -> Result<(String, String), Error> {
        let response = self
            .post()
            .header("type", "AuthSmsSendTo")
            .header("access", &self.session.refresh)
            .header("refresh", &self.session.refresh)
            .send()?;
        handle_error!(response);
        let phone = get_header(&response, "phone");
        let message = get_header(&response, "message");
        Ok((phone, message))
    }
}

impl Default for Client {
    fn default() -> Self {
        let config = Config::load();
        let client = Client {
            config,
            session: Session { access: "".into(), refresh: "".into() },
        };
        //TODO update session
        client
    }
}

fn get_header(response: &Response, key: &str) -> String {
    response.headers()[key]
        .to_str()
        .unwrap_or_default()
        .to_string()
}
