pub struct Config {
    pub url: String,
    session: Option<Session>,
    plan: Plan,
}

struct Session {
    access: Option<String>,
    refresh: Option<String>,
}

struct Plan {
    time: u64,
    space: u64,
    traffic: u64,
    tips: u64,
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
