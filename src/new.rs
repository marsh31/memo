use chrono::{Datelike, Local};
use std::os::unix::process::CommandExt;
use std::process::Command;

pub struct NewParam {
    pub root: String,
    pub filename: String,
    pub extension: String,
    pub format: String,
}

pub fn new(param: NewParam) {
    let editor_cmd = get_editor_env_variable_or_default("vim");
    let filename = get_filename(&param);

    run_cmd(&editor_cmd, &filename);
}

fn get_filename(param: &NewParam) -> String {
    let data = Local::now();
    let mut root = param.root.clone();
    let mut result = param.format.clone();

    if !root.ends_with("/") {
        root.push('/');
    }

    result = result.replace("{year}", &format!("{}", &data.year()));
    result = result.replace("{Mon}", &format!("{:02}", &data.month()));
    result = result.replace("{Day}", &format!("{:02}", &data.day()));
    result = result.replace("{name}", &format!("{}", &param.filename));
    result = result.replace("{extension}", &format!("{}", &param.extension));

    format!("{}{}", root, result)
}

fn get_editor_env_variable_or_default(default: &str) -> String {
    match std::env::var("EDITOR") {
        Ok(editor) => editor,
        Err(_) => default.to_string(),
    }
}

fn run_cmd(cmd: &str, args: &str) {
    let _ = Command::new(cmd).arg(args).exec();
}

#[cfg(test)]
mod tests {
    use super::{get_editor_env_variable_or_default, get_filename, NewParam};

    #[test]
    fn env() {
        assert_eq!("vim", get_editor_env_variable_or_default("vim"));
    }

    #[test]
    fn get_filename_test() {
        let param = NewParam {
            root: "/home/marsh/src/test".to_string(),
            filename: "test".to_string(),
            extension: "md".to_string(),
            format: "{year}-{Mon}-{Day}@{name}.{extension}".to_string(),
        };

        let filename = get_filename(&param);
        assert_eq!("/home/marsh/src/test/2021-01-05@test.md", filename)
    }

    #[test]
    fn string_push() {
        let mut string = "test".to_string();
        string.push('/');

        assert_eq!("test/", string);
        assert!(string.ends_with("/"));
    }
}
