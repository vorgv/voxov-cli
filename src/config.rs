pub struct Config {
    pub url: String,
    pub session: Option<Session>,
    pub plan: Plan,
}

pub struct Session {
    pub access: String,
    pub refresh: String,
}

pub struct Plan {
    pub time: u64,
    pub space: u64,
    pub traffic: u64,
    pub tips: u64,
}

impl Config {
    pub fn load() -> Self {
        //TODO support config file on linux, windows, macos
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        let default_cost = 1_000_000_000_u64;
        Config {
            url: "localhost:8080".into(),
            session: None,
            plan: Plan {
                time: default_cost,
                space: default_cost,
                traffic: default_cost,
                tips: default_cost,
            },
        }
    }
}
