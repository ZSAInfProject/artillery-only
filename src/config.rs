use serde::Deserialize;
use std::net::Ipv4Addr;

#[derive(Debug, Deserialize, PartialEq)]
struct NetworkConfig {
    host: Ipv4Addr,
    port: u16,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GameConfig {
    pub player_count: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ServerConfig {
    network: NetworkConfig,
    pub game: GameConfig,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ClientConfig {
    network: NetworkConfig,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub server: Option<ServerConfig>,
    pub client: Option<ClientConfig>,
}

impl Config {
    pub fn new<T: Iterator<Item = String>>(args: T) -> Result<Self, config::ConfigError> {
        let mut s = config::Config::new();
        let mut args = args.skip(1);
        s.merge(config::File::with_name("config/default.json"))
            .unwrap();

        let mut s: Result<Config, config::ConfigError> = s.try_into();

        if let Ok(ref mut s) = s {
            if let Some(run_as) = args.next() {
                match run_as.as_ref() {
                    "server" => {
                        s.client = None;
                    }
                    "client" => {
                        s.server = None;
                    }
                    _ => (),
                }
            }
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Had to use assert_ne! instead of assert_eq! because in asserts pattern matching doesnt work

    #[test]
    fn both_launch_mode() {
        let args = vec!["program_name".to_owned(), "both".to_owned()].into_iter();

        let config = Config::new(args).unwrap();
        let (server, client) = (config.server, config.client);

        assert_ne!(server, None);
        assert_ne!(client, None);
    }

    #[test]
    fn client_launch_mode() {
        let args = vec!["program_name".to_owned(), "client".to_owned()].into_iter();

        let config = Config::new(args).unwrap();
        let (server, client) = (config.server, config.client);

        assert_eq!(server, None);
        assert_ne!(client, None);
    }

    #[test]
    fn server_launch_mode() {
        let args = vec!["program_name".to_owned(), "server".to_owned()].into_iter();

        let config = Config::new(args).unwrap();
        let (server, client) = (config.server, config.client);

        assert_ne!(server, None);
        assert_eq!(client, None);
    }

    #[test]
    fn unspecified_launch_mode() {
        let args = vec!["program_name".to_owned()].into_iter();

        let config = Config::new(args).unwrap();
        let (server, client) = (config.server, config.client);

        assert_ne!(server, None);
        assert_ne!(client, None);
    }
}
