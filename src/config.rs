use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::utils::expand;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub root: String,
    pub extension: String,
}

pub fn load_config(files: Vec<&str>) -> Config {
    let default: String = r#"
root = "$HOME"
extension = "md"
    "#
    .to_string();

    let mut config: Config = match check_config_file(files) {
        Some(filename) => {
            let read_config_str = fs::read_to_string(filename).unwrap();
            match toml::from_str(&read_config_str) {
                Ok(res) => res,
                Err(why) => {
                    println!("Err: {:?}", why.to_string());
                    toml::from_str(&default).unwrap()
                }
            }
        }
        None => toml::from_str(&default).unwrap(),
    };

    config.root = expand(&config.root);
    config
}

fn check_config_file(files: Vec<&str>) -> Option<String> {
    for file in files.clone().iter() {
        let file = expand(file);
        if Path::new(&file).exists() {
            return Some(file);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_config_file_test() {
        let files = vec![
            "~/.memo.toml",
            "~/.config/memo.toml",
            "./resource/memo.toml",
        ];

        let expect = check_config_file(files).unwrap();
        assert_eq!(expect, "./resource/memo.toml".to_string());
    }

    #[test]
    fn check_config_file_test2() {
        let files = vec!["~/.memo.toml", "~/.config/memo.toml"];

        let expect = check_config_file(files);
        assert_eq!(expect, None);
    }

    #[test]
    fn check_config_file_test3() {
        let files = vec!["$HOME/.memo.toml"];

        let expect = check_config_file(files);
        assert_eq!(expect, None);
    }

    #[test]
    fn check_config_file_test4() {
        let files = vec![
            "~/.memo.toml",
            "~/.config/memo.toml",
            "$HOME/src/rust/memo/resource/memo.toml",
        ];

        let expect = check_config_file(files).unwrap();
        assert_eq!(
            expect,
            "/home/marsh/src/rust/memo/resource/memo.toml".to_string()
        );
    }

    #[test]
    fn load_config_test() {
        let files = vec![
            "~/.memo.toml",
            "~/.config/memo.toml",
            "./resource/memo.toml",
        ];

        let config = load_config(files);
        assert_eq!(config.root, "/home/marsh/Memo");
        assert_eq!(config.extension, "md");
    }

    #[test]
    fn load_config_test2() {
        let files = vec!["~/.memo.toml", "~/.config/memo.toml"];

        let config = load_config(files);
        assert_eq!(config.root, "/home/marsh");
        assert_eq!(config.extension, "md");
    }
}
