use std::env::Args;
use std::error::Error;
use std::fmt;
use std::net::AddrParseError;
use std::num::ParseIntError;
use std::process;

#[derive(Debug)]
pub struct GameConfig {
    pub player_count: u32,
}

#[derive(Debug)]
pub struct ServerConfig {
    network_config: enet::Address,
    pub game_config: GameConfig,
}

#[derive(Debug)]
pub struct ClientConfig {
    network_config: enet::Address,
}

#[derive(Debug)]
pub struct Config {
    pub server_config: Option<ServerConfig>,
    pub client_config: Option<ClientConfig>,
}

#[derive(Debug)]
pub enum ConfigParseError {
    LaunchModeError,
    NumParseError(ParseIntError),
    IpParseError(AddrParseError),
}

impl Error for ConfigParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use ConfigParseError::*;
        match self {
            LaunchModeError => None,
            NumParseError(e) => Some(e),
            IpParseError(e) => Some(e),
        }
    }
}

impl From<ParseIntError> for ConfigParseError {
    fn from(e: ParseIntError) -> ConfigParseError {
        ConfigParseError::NumParseError(e)
    }
}

impl From<AddrParseError> for ConfigParseError {
    fn from(e: AddrParseError) -> ConfigParseError {
        ConfigParseError::IpParseError(e)
    }
}

impl fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ConfigParseError::*;
        match *self {
            LaunchModeError => write!(f, "Invalid launch mode. Valid modes: [client|server|both]"),
            NumParseError(ref error) => error.fmt(f),
            IpParseError(ref error) => error.fmt(f),
        }
    }
}

impl Config {
    pub fn new(args: Args) -> Result<Config, ConfigParseError> {
        let mut config = Config {
            server_config: None,
            client_config: None,
        };

        let mut args = args.skip(1);
        let run_as = args.next().unwrap_or_else(|| {
            println!("Usage: 'artillery-only client/server/both'");
            process::exit(1);
        });

        let host = args.next().unwrap().parse()?;
        let port = args.next().unwrap().parse()?;

        let network_config = enet::Address::new(host, port);

        let player_count = args.next().unwrap().parse()?;
        let game_config = GameConfig { player_count };

        let server_config = ServerConfig {
            network_config: network_config.clone(),
            game_config,
        };

        let client_config = ClientConfig {
            network_config: network_config.clone(),
        };

        match run_as.as_ref() {
            "client" => config.client_config = Some(client_config),
            "server" => config.server_config = Some(server_config),
            "both" => {
                config.client_config = Some(client_config);
                config.server_config = Some(server_config);
            }
            _ => return Err(ConfigParseError::LaunchModeError),
        }

        Ok(config)
    }
}
