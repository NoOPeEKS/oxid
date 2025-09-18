use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Config {
    pub lsp: Vec<LspConfig>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct LspConfig {
    pub filetype: String,
    pub command: String,
}

pub fn read_config_file() -> anyhow::Result<Config> {
    #[allow(deprecated)]
    match std::env::home_dir() {
        Some(dir) => {
            let cfg_path = dir.join(".config").join("oxid").join("oxid.toml");
            match std::fs::read_to_string(cfg_path) {
                Ok(cfg_str) => match toml::from_str::<Config>(&cfg_str) {
                    Ok(config) => Ok(config),
                    Err(_) => anyhow::bail!("Could not parse oxid.toml"),
                },
                Err(_) => anyhow::bail!("Could not read oxid.toml"),
            }
        }
        None => anyhow::bail!("Could not find $HOME directory."),
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self},
        io::Write,
        path::PathBuf,
    };

    use super::*;

    fn config_dir() -> PathBuf {
        std::env::home_dir().unwrap().join(".config").join("oxid")
    }
    fn config_path() -> PathBuf {
        config_dir().join("oxid.toml")
    }

    #[test]
    fn test_read_config_file() {
        let toml_str = r#"
            [[lsp]]
            filetype = "rust"
            command = "rust-analyzer"

            [[lsp]]
            filetype = "python"
            command = "pyrefly lsp"

            [[lsp]]
            filetype = "gleam"
            command = "gleam lsp"
        "#;

        let cfg_dir = config_dir();
        let cfg_path = config_path();

        fs::create_dir_all(&cfg_dir).unwrap();

        let mut file = fs::File::create(&cfg_path).unwrap();
        file.write_all(toml_str.as_bytes()).unwrap();

        let cfg = read_config_file().unwrap();

        assert_eq!(
            cfg,
            Config {
                lsp: vec![
                    LspConfig {
                        filetype: "rust".to_string(),
                        command: "rust-analyzer".to_string()
                    },
                    LspConfig {
                        filetype: "python".to_string(),
                        command: "pyrefly lsp".to_string()
                    },
                    LspConfig {
                        filetype: "gleam".to_string(),
                        command: "gleam lsp".to_string()
                    }
                ]
            }
        );

        fs::remove_file(&cfg_path).unwrap();
        fs::remove_dir_all(&cfg_dir).unwrap();
    }
}
