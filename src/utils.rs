use regex::Regex;

pub fn expand(string: &str) -> String {
    match string.find("$") {
        Some(_) => {
            let mut ret: String = string.to_string().clone();

            let re = Regex::new("(\\$[a-zA-Z_]+[a-zA-Z0-9_]*)").unwrap();
            for cap in re.captures_iter(string) {
                let env_var = &cap[1].to_string();
                match std::env::var(env_var[1..].to_string()) {
                    Ok(result) => {
                        ret = ret.replace(env_var, &result);
                    }
                    Err(_) => {}
                }
            }

            ret
        }
        None => string.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_test() {
        let path = expand("$HOME/src/test");
        assert_eq!("/home/marsh/src/test", path);

        let txt = expand("home is $HOME. editor is $EDITOR");
        assert_eq!("home is /home/marsh. editor is vim", txt);
    }

    #[test]
    fn utils_test() {
        let string = "$HOME/test";

        assert_eq!(string.find("$").unwrap(), 0);
        assert_eq!(string.find("#"), None);
    }
}
