extern crate regex;
extern crate serde;
extern crate toml;

use clap::{crate_authors, crate_version, App, AppSettings, Arg};

mod config;
mod list;
mod new;
mod utils;

fn main() {
    let matches = App::new("memo")
        .setting(AppSettings::ArgRequiredElseHelp)
        .about("Memo tool")
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(
            App::new("new").about("create a new memo file").arg(
                Arg::new("file")
                    .about("The memo file name")
                    .required(false)
                    .min_values(0),
            ),
        )
        .subcommand(App::new("list").about("show a list of memo files"))
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .takes_value(true)
                .required(false)
                .about("Set config file path"),
        )
        .get_matches();

    let mut default_config_path_list = vec!["$HOME/.memo.toml", "$HOME/.config/memo.toml"];
    let config_file_path_list: Vec<&str> = match matches.value_of("config") {
        Some(value) => {
            default_config_path_list.insert(0, value);
            default_config_path_list
        }
        None => default_config_path_list,
    };

    let config = config::load_config(config_file_path_list);

    match matches.subcommand() {
        Some(("new", new_matches)) => {
            let filename = match new_matches.values_of("file") {
                Some(values) => {
                    let files: Vec<_> = values.collect();
                    files.join("_").to_string()
                }
                None => "".to_string(),
            };

            let param = new::NewParam {
                root: config.root,
                filename,
                extension: config.extension,
                format: "{year}-{Mon}-{Day}@{name}.{extension}".to_string(),
            };

            new::new(param);
        }

        Some(("list", _)) => {
            let param = list::ListParam {
                root: config.root,
                extension: config.extension,
            };
            list::list(param);
        }

        None => {
            println!("no matches")
        }
        _ => unreachable!(),
    }
}
