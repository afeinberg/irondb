use config::{Config, Environment, File};

pub struct ClusterConfig {
    dflt_server_hostport: String,
}

impl ClusterConfig {
    pub fn from(name: &str) -> ClusterConfig {
        let mut settings = Config::default();
        settings
            .merge(File::with_name(name))
            .unwrap()
            .merge(Environment::with_prefix("IRONDB"))
            .unwrap();

        ClusterConfig {
            dflt_server_hostport: settings.get("dflt_server_hostport").unwrap(),
        }
    }

    pub fn dflt_server_hostport(&self) -> String {
        self.dflt_server_hostport.clone()
    }
}
