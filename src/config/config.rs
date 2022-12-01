use std::{fs::File, io::Read};

use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::util::path;

use super::env::get_env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub google_oauth: GoogleOAuthConfig,
    pub auth: AuthConfig,
    pub db: DbConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GoogleOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub login_redirect_url: String,
    pub create_user_redirect_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DbConfig {
    pub db_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub cookie_secret: String,
}

static CONFIG: Lazy<Config> = Lazy::new(Config::new);

impl Config {
    pub fn get() -> &'static Self {
        &CONFIG
    }
    // pub fn get_or_init() -> &'static Config {
    //     &CONFIG.get_or_init(|| Config::new())
    // }

    fn new() -> Self {
        let env = get_env();
        let toml_string = Config::read_config_file_to_string(env, "toml");

        toml::from_str(&toml_string).expect("Failed to parse config file")
    }

    fn read_config_file_to_string(file_name: &str, file_type: &str) -> String {
        /*
            CONFIG FILE PATH
        */
        let config_file_path = Config::config_file_path(file_name, file_type);
        println!("{config_file_path}");
        /*
            READ CONFIG FILE TO STRING
        */
        let mut f = File::open(config_file_path).expect("failed opening `config` file");
        let mut buf = String::new();
        f.read_to_string(&mut buf)
            .expect("failed reading `config` file");

        buf.trim().to_string()
    }

    fn config_file_path(file_name: &str, file_type: &str) -> String {
        let config_dir_name = Config::config_dir_path();
        format!("{config_dir_name}/.{file_name}.{file_type}")
    }

    fn config_dir_path() -> String {
        let project_root = path::project_root();
        format!("{project_root}/src/config")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config__dir_path() {
        /*
            env: TEST
        */
        let path = Config::config_dir_path();
        println!("{}", path);
        assert_eq!(
            "/Users/undervars/My Drive/dev/underair/server_rs/src/config",
            path
        );
    }

    #[test]
    fn test_read_config_file_to_string() {
        /*
            READ `test.toml` FILE TO STRING
            AND WRITE THE CONTENTS OF IT TO A NEW FILE `__TEST__.toml`
        */
        let env = get_env();
        let config_string = Config::read_config_file_to_string(env, "toml");

        assert_ne!(config_string.len(), 0);
    }

    #[test]
    fn test_get_or_init_config() {
        let c = Config::get();
        assert_eq!(c.server.host, "localhost");
        assert_eq!(c.server.port, 4000);
        assert_eq!(
            c.google_oauth.client_id,
            "775415266610-insjflo30jv4iunv9dmqu98pnp2tqnfv.apps.googleusercontent.com"
        );
        assert_eq!(c.google_oauth.client_secret, "lOdZD97VRgJj-KBe2INp85oX");
        assert_eq!(
            c.google_oauth.login_redirect_url,
            "http://localhost:4000/auth/google/login/redirect"
        );
        assert_eq!(
            c.google_oauth.create_user_redirect_url,
            "http://localhost:4000/auth/google/create_user/redirect"
        );
        assert_eq!(c.auth.jwt_secret, "secret");
        assert_eq!(c.auth.cookie_secret, "secret");
    }
}
