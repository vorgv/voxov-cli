use chrono::{DateTime, Duration, Utc};

pub struct Session {
    pub access: String,
    pub refresh: String,
    pub access_utc: String,
    pub refresh_utc: String,
    pub access_minutes: i64,
    pub refresh_days: i64,
}

impl Session {
    pub fn new(access: &str, refresh: &str) -> Self {
        let now = Utc::now().to_rfc3339();
        Session {
            access: access.into(),
            refresh: refresh.into(),
            access_utc: now.clone(),
            refresh_utc: now,
            access_minutes: 60,
            refresh_days: 30,
        }
    }

    pub fn set_access(&mut self, access: &str) {
        self.access = access.into();
        self.access_utc = Utc::now().to_rfc3339();
    }

    pub fn set_refresh(&mut self, refresh: &str) {
        self.refresh = refresh.into();
        self.refresh_utc = Utc::now().to_rfc3339();
    }

    pub fn access_expired(&self) -> bool {
        let then = DateTime::parse_from_rfc3339(&self.access).unwrap();
        Utc::now() > then + Duration::minutes(self.access_minutes)
    }

    pub fn refresh_expired(&self) -> bool {
        let then = DateTime::parse_from_rfc3339(&self.refresh).unwrap();
        Utc::now() > then + Duration::days(self.refresh_days)
    }

    pub fn needs_refresh(&self) -> bool {
        let then = DateTime::parse_from_rfc3339(&self.access).unwrap();
        Utc::now() > then + Duration::minutes(self.access_minutes / 2)
    }
}

pub struct Plan {
    pub time: u64,
    pub space: u64,
    pub traffic: u64,
    pub tips: u64,
}

pub struct Config {
    pub url: String,
    pub session: Option<Session>,
    pub plan: Plan,
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
            url: "http://localhost:8080".into(),
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
