use std::{
    io::{stdin, stdout, Read, Write},
    process,
};

use reqwest::{
    blocking::{get, Client as ReqwestClient, RequestBuilder, Response},
    Error,
};

use crate::config::{Config, Session};

pub struct Client {
    config: Config,
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

    /// Refresh or remake session.
    fn update_session(&mut self) {
        match &self.config.session {
            Some(s) if !s.refresh_expired() => {
                if s.needs_refresh() {
                    let access = self.auth_session_refresh().unwrap();
                    self.config.session.as_mut().unwrap().set_access(&access);
                }
            }
            x => {
                if x.is_some() {
                    eprintln!("Refresh token expired. Session is reset for re-authentication.");
                }
                let (access, refresh) = self.auth_session_start().unwrap();
                self.config.session = Some(Session::new(&access, &refresh));
            }
        };
    }

    /// Authenticate.
    pub fn auth(&self) -> Result<String, Error> {
        let (phone, message) = self.auth_sms_send_to()?;
        println!("Send SMS message {} to {}.", message, phone);
        println!("Press any key after sent.");
        let mut stdout = stdout();
        stdout.write_all(b"Press Enter to continue...").unwrap();
        stdout.flush().unwrap();
        stdin().read_exact(&mut [0]).unwrap();
        let uid = self.auth_sms_sent(&phone, &message)?;
        Ok(format!("Your user ID is {}", uid))
    }

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
            .header("refresh", &self.config.session.as_ref().unwrap().refresh)
            .send()?;
        handle_error!(response);
        let access = get_header(&response, "access");
        Ok(access)
    }

    pub fn auth_session_end(&self, drop_refresh: bool) -> Result<(), Error> {
        let mut builder = self
            .post()
            .header("type", "AuthSessionEnd")
            .header("access", &self.config.session.as_ref().unwrap().access);
        if drop_refresh {
            builder = builder.header("refresh", &self.config.session.as_ref().unwrap().refresh);
        }
        let response = builder.send()?;
        handle_error!(response);
        Ok(())
    }

    pub fn auth_sms_send_to(&self) -> Result<(String, String), Error> {
        let response = self
            .post()
            .header("type", "AuthSmsSendTo")
            .header("access", &self.config.session.as_ref().unwrap().refresh)
            .header("refresh", &self.config.session.as_ref().unwrap().refresh)
            .send()?;
        handle_error!(response);
        let phone = get_header(&response, "phone");
        let message = get_header(&response, "message");
        Ok((phone, message))
    }

    pub fn auth_sms_sent(&self, phone: &str, message: &str) -> Result<String, Error> {
        let response = self
            .post()
            .header("type", "AuthSmsSent")
            .header("access", &self.config.session.as_ref().unwrap().refresh)
            .header("refresh", &self.config.session.as_ref().unwrap().refresh)
            .header("phone", phone)
            .header("message", message)
            .send()?;
        handle_error!(response);
        let uid = get_header(&response, "uid");
        Ok(uid)
    }
}

impl Default for Client {
    fn default() -> Self {
        let config = Config::load();
        let mut client = Client { config };
        client.update_session();
        client
    }
}

fn get_header(response: &Response, key: &str) -> String {
    response.headers()[key]
        .to_str()
        .unwrap_or_default()
        .to_string()
}
