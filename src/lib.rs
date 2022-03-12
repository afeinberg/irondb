pub mod core;
pub mod store;
pub mod util;
pub mod version;

#[cfg(test)]
mod tests {
    use crate::core::ClusterConfig;

    #[test]
    fn test_dflt_server_hostport() {
        let cfg = ClusterConfig::from("Cluster");
        assert_eq!(cfg.dflt_server_hostport(), "[::1]:10081")
    }
}
