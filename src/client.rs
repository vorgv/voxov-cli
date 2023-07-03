use std::{
    fs::File,
    io::{stdin, Write, BufRead},
    process,
};

use reqwest::{
    blocking::{get, Client as ReqwestClient, RequestBuilder, Response},
    Error,
};

use crate::config::{Config, Plan, Session};

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

macro_rules! generate_utils {
    ($s:expr) => {
        fn usage_exit() -> ! {
            eprintln!($s);
            process::exit(1);
        }
        fn assert_min_len(v: &[String], n: usize) {
            if v.len() < n {
                usage_exit();
            }
        }
        fn assert_has_len(v: &[String], n: usize) {
            if v.len() != n {
                usage_exit();
            }
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

    /// Post, but with head included.
    fn post_head(&self, fed: Option<String>) -> RequestBuilder {
        let mut builder = self
            .post()
            .header("access", &self.config.session.as_ref().unwrap().access)
            .header("time", self.config.plan.time.to_string())
            .header("space", self.config.plan.space.to_string())
            .header("traffic", self.config.plan.traffic.to_string())
            .header("tips", self.config.plan.tips.to_string());
        if let Some(f) = fed {
            builder = builder.header("fed", f);
        }
        builder
    }

    /// Refresh or remake session.
    fn update_session(&mut self) {
        match &self.config.session {
            Some(s) if !s.refresh_expired() => {
                if s.needs_refresh() {
                    let access = self.auth_session_refresh().unwrap();
                    self.config.session.as_mut().unwrap().set_access(&access);
                    self.config.save();
                }
            }
            x => {
                if x.is_some() {
                    eprintln!("Refresh token expired. Session is reset for re-authentication.");
                }
                let (access, refresh) = self.auth_session_start().unwrap();
                self.config.session = Some(Session::new(&access, &refresh));
                self.config.save();
            }
        };
    }

    /// Authenticate.
    pub fn auth(&self) -> Result<String, Error> {
        let (phone, message) = self.auth_sms_send_to()?;
        println!("Send SMS message {} to {}.", message, phone);
        println!("Press enter after sent.");
        let mut s = "".to_string();
        let _ = stdin().read_line(&mut s);
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
            .header("access", &self.config.session.as_ref().unwrap().access)
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
            .header("access", &self.config.session.as_ref().unwrap().access)
            .header("refresh", &self.config.session.as_ref().unwrap().refresh)
            .header("phone", phone)
            .header("message", message)
            .send()?;
        handle_error!(response);
        let uid = get_header(&response, "uid");
        Ok(uid)
    }

    /// Manage credit.
    pub fn cost(&self, action: &str) -> Result<String, Error> {
        match action {
            "pay" => self.cost_pay(),
            "get" => self.cost_get(),
            _ => {
                eprintln!("cost pay|get");
                process::exit(1);
            }
        }
    }

    pub fn cost_pay(&self) -> Result<String, Error> {
        let response = self
            .post()
            .header("type", "CostPay")
            .header("access", &self.config.session.as_ref().unwrap().access)
            .header("vendor", "00000000000000000000000000000000")
            .send()?;
        handle_error!(response);
        let uri = get_header(&response, "uri");
        Ok(uri)
    }

    pub fn cost_get(&self) -> Result<String, Error> {
        let response = self
            .post()
            .header("type", "CostGet")
            .header("access", &self.config.session.as_ref().unwrap().access)
            .send()?;
        handle_error!(response);
        let credit = get_header(&response, "credit");
        Ok(credit)
    }

    pub fn print_cost(&self, response: &Response) {
        macro_rules! get {
            ($s:expr) => {
                get_header(response, $s).parse().unwrap()
            };
        }
        let changes = Plan {
            time: get!("time"),
            space: get!("space"),
            traffic: get!("traffic"),
            tips: get!("tips"),
        };
        let plan = &self.config.plan;
        println!(
            "time {} space {} traffic {} tips {}",
            plan.time - changes.time,
            plan.space - changes.space,
            plan.traffic - changes.traffic,
            plan.tips - changes.tips
        );
    }

    /// Call functions.
    pub fn gene(&self, args: &[String]) -> Result<String, Error> {
        generate_utils!("gene [fed FID] (meta GID|call GID [ARG])");
        assert_min_len(args, 3);
        let (fed, method) = match args[2].as_str() {
            "fed" => {
                assert_min_len(args, 5);
                (Some(args[3].clone()), &args[4..])
            }
            "meta" => (None, &args[2..]),
            "call" => (None, &args[2..]),
            _ => usage_exit(),
        };
        let builder = self.post_head(fed);
        match method[0].as_str() {
            "meta" => {
                assert_has_len(method, 2);
                let response = builder
                    .header("type", "GeneMeta")
                    .header("gid", &method[1])
                    .send()?;
                handle_error!(response);
                self.print_cost(&response);
                response.text()
            }
            "call" => {
                assert_min_len(method, 2);
                let response = builder
                    .header("type", "GeneCall")
                    .header("gid", &method[1])
                    .header(
                        "arg",
                        match method.len() {
                            2 => "".into(),
                            _ => method[2..].join(" "),
                        },
                    )
                    .send()?;
                handle_error!(response);
                self.print_cost(&response);
                response.text()
            }
            _ => usage_exit(),
        }
    }

    /// Read write data.
    pub fn meme(&self, args: &[String]) -> Result<String, Error> {
        generate_utils!("meme (meta HASH|raw-put DAYS FILE|raw-get [-p] HASH)");
        let mut builder = self.post_head(None);
        assert_min_len(args, 3);
        match args[2].as_str() {
            "meta" => {
                assert_has_len(args, 4);
                let response = builder
                    .header("type", "MemeMeta")
                    .header("hash", &args[3])
                    .send()?;
                handle_error!(response);
                self.print_cost(&response);
                response.text()
            }
            "raw-put" => {
                assert_min_len(args, 4);
                builder = builder
                    .header("type", "MemeRawPut")
                    .header("days", &args[3]);
                builder = if args.len() >= 5 {
                    assert_has_len(args, 5);
                    let file = File::open(&args[4]).unwrap();
                    builder.body(file)
                } else {
                    builder.body(std::io::stdin().lock().lines().fold("".to_string(), |acc, line| {
        acc + &line.unwrap() + "\n"
    }))
                };
                let response = builder.send()?;
                handle_error!(response);
                self.print_cost(&response);
                let hash = get_header(&response, "hash");
                Ok(hash)
            }
            "raw-get" => {
                assert_min_len(args, 4);
                let mut to_file = false;
                builder = match args[3].as_str() {
                    "-p" => {
                        assert_min_len(args, 5);
                        if args.len() >= 5 {
                            assert_has_len(args, 6);
                            to_file = true;
                        }
                        builder.header("public", "true").header("hash", &args[4])
                    }
                    _ => {
                        if args.len() >= 4 {
                            assert_has_len(args, 5);
                            to_file = true;
                        }
                        builder.header("public", "false").header("hash", &args[3])
                    }
                };
                let response = builder.send()?;
                handle_error!(response);
                self.print_cost(&response);
                if to_file {
                    let filename = &args[args.len() - 1];
                    let mut file = File::create(filename).unwrap();
                    file.write_all(&response.bytes().unwrap()).unwrap();
                    Ok("".into())
                } else {
                    response.text()
                }
            }
            _ => usage_exit(),
        }
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
