use std::{fs, sync::OnceLock};

use serde::{Deserialize, Serialize};
use tracing::{error, warn};

static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    discord: Option<ConfigTomlDiscord>,
    youtube: Option<ConfigTomlYoutube>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlDiscord {
    bot_token: Option<String>,
    guild_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlYoutube {
    project_id: Option<String>,

    #[serde(rename = "type")]
    auth_type: Option<String>,
    private_key_id: Option<String>,
    private_key: Option<String>,
    client_email: Option<String>,
    client_id: Option<String>,
    auth_uri: Option<String>,
    token_uri: Option<String>,
    auth_provider_x509_cert_url: Option<String>,
    client_x509_cert_url: Option<String>,
    universe_domain: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub discord_token: String,
    pub guild_id: String,

    pub youtube_project_id: String,
    pub youtube_auth_type: String,
    pub youtube_private_key_id: String,
    pub youtube_private_key: String,
    pub youtube_client_email: String,
    pub youtube_client_id: String,
    pub youtube_auth_uri: String,
    pub youtube_token_uri: String,
    pub youtube_auth_provider_x509_cert_url: String,
    pub youtube_client_x509_cert_url: String,
    pub _youtube_universe_domain: String,
}

impl Config {
    pub fn new() -> Self {
        #[cfg(debug_assertions)]
        let config_filepaths = vec![
            "./secret/config_dev.toml",
            "./secret/config_release.toml",
            "./config.toml",
            "./Config.toml",
        ];
        #[cfg(not(debug_assertions))]
        let config_filepaths = vec!["./config.toml", "./Config.toml"];

        let mut file_content = "".to_owned();

        for filepath in config_filepaths {
            match fs::read_to_string(filepath) {
                Ok(content) => {
                    file_content = content;
                    break;
                }
                Err(_err) => {
                    error!("cant find in {filepath}");
                }
            }
        }

        let config_toml: ConfigToml = toml::from_str(&file_content).unwrap_or_else(|_| {
            error!("Failed to create ConfigToml Object out of config file.");
            ConfigToml {
                discord: None,
                youtube: None,
            }
        });

        let (discord_token, guild_id) = match config_toml.discord {
            Some(discord) => {
                let discord_token = discord.bot_token.unwrap_or_else(|| {
                    panic!("Missing discord bot token field");
                });
                let guild_id = discord.guild_id.unwrap_or_else(|| {
                    panic!("Missing discord guild_id field");
                });
                (discord_token, guild_id)
            }
            None => {
                panic!("MISSING DISCORD CONFIG");
            }
        };

        let (
            youtube_project_id,
            youtube_auth_type,
            youtube_private_key_id,
            youtube_private_key,
            youtube_client_email,
            youtube_client_id,
            youtube_auth_uri,
            youtube_token_uri,
            youtube_auth_provider_x509_cert_url,
            youtube_client_x509_cert_url,
            _youtube_universe_domain,
        ) = match config_toml.youtube {
            Some(yt) => {
                let client_id = yt.client_id.unwrap_or_else(|| {
                    warn!("Missing youtube client_id in config file, wont be able to search youtube links probably");
                    "unknown".to_owned()
                });
                let auth_type = yt.auth_type.unwrap_or_else(|| {
                    warn!("Missing youtube auth_type in config file, wont be able to search youtube links probably");
                    "unknown".to_owned()
                });
                let private_key_id = yt.private_key_id.unwrap_or_else(|| {
                    warn!("Missing youtube private_key_id in config file, wont be able to search youtube links probably");
                    "unknown".to_owned()
                });
                let private_key = yt.private_key.unwrap_or_else(|| {
                    warn!("Missing youtube private_key in config file, wont be able to search youtube links probably");
                    "unknown".to_owned()
                });
                let client_email = yt.client_email.unwrap_or_else(|| {
                    warn!("Missing youtube client_email in config file, wont be able to search youtube links probably");
                    "unknown".to_owned()
                });
                let project_id = yt.project_id.unwrap_or_else(|| {
                    warn!("Missing youtube project_id in config file, wont be able to search youtube links probably");
                    "unknown".to_owned()
                });
                let auth_uri = yt.auth_uri.unwrap_or_else(|| {
                    warn!("Missing youtube auth_uri in config file, wont be able to search youtube links probably");
                    "unknown".to_owned()
                });
                let token_uri = yt.token_uri.unwrap_or_else(|| {
                    warn!("Missing youtube token_uri in config file, wont be able to search youtube links probably");
                    "unknown".to_owned()
                });
                let auth_provider_x509_cert_url = yt.auth_provider_x509_cert_url.unwrap_or_else(|| {
                    warn!("Missing youtube auth_provider_x509_cert_url in config file, wont be able to search youtube links probably");
                    "unknown".to_owned()
                });
                let client_x509_cert_url = yt.client_x509_cert_url.unwrap_or_else(|| {
                    warn!("Missing youtube client_x509_cert_url in config file, wont be able to search youtube links probably");
                    "unknown".to_owned()
                });
                let universe_domain = yt.universe_domain.unwrap_or_else(|| {
                    warn!("Missing youtube universe_domain in config file, wont be able to search youtube links probably");
                    "unknown".to_owned()
                });

                (
                    project_id,
                    auth_type,
                    private_key_id,
                    private_key,
                    client_email,
                    client_id,
                    auth_uri,
                    token_uri,
                    auth_provider_x509_cert_url,
                    client_x509_cert_url,
                    universe_domain,
                )
            }
            None => (
                "unknown".to_owned(),
                "unknown".to_owned(),
                "unknown".to_owned(),
                "unknown".to_owned(),
                "unknown".to_owned(),
                "unknown".to_owned(),
                "unknown".to_owned(),
                "unknown".to_owned(),
                "unknown".to_owned(),
                "unknown".to_owned(),
                "unknown".to_owned(),
            ),
        };

        Self {
            discord_token,
            guild_id,
            youtube_project_id,
            youtube_auth_type,
            youtube_private_key_id,
            youtube_private_key,
            youtube_client_email,
            youtube_client_id,
            youtube_auth_uri,
            youtube_token_uri,
            youtube_auth_provider_x509_cert_url,
            youtube_client_x509_cert_url,
            _youtube_universe_domain,
        }
    }

    pub fn get() -> &'static Self {
        GLOBAL_CONFIG.get_or_init(|| {
            let config = Config::new();
            // info!("{config:#?}");
            config
        })
    }
}
