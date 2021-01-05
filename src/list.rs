use std::fs;

pub struct ListParam {
    pub root: String,
    pub extension: String,
}

pub fn list(param: ListParam) {
    let list = get_file_list(&param.root, true);

    match list {
        Ok(list) => {
            let mut res: Vec<_> = list
                .iter()
                .filter(|x| x.ends_with(&(".".to_string() + &param.extension)))
                .collect();

            res.sort();

            for item in res.clone().iter() {
                println!("{}", item);
            }
        }
        Err(why) => {
            println!("{}", why)
        }
    }
}

fn get_file_list(root: &str, is_recursive: bool) -> Result<Vec<String>, String> {
    match fs::read_dir(root) {
        Ok(paths) => {
            let mut store: Vec<String> = vec![];

            for path in paths {
                let path = path.unwrap();
                let meta = path.metadata().unwrap();

                if meta.is_dir() && is_recursive {
                    get_file_list_sub(path.path().to_str().unwrap(), &mut store);
                } else if meta.is_file() {
                    store.push(path.path().to_str().unwrap().to_string());
                }
            }
            Ok(store)
        }
        Err(why) => Err(format!("Err: {:?}", why.kind())),
    }
}

fn get_file_list_sub(root: &str, store: &mut Vec<String>) {
    match fs::read_dir(root) {
        Ok(paths) => {
            for path in paths {
                let path = path.unwrap();
                let meta = path.metadata().unwrap();

                if meta.is_dir() {
                    get_file_list_sub(path.path().to_str().unwrap(), store);
                } else if meta.is_file() {
                    store.push(path.path().to_str().unwrap().to_string());
                }
            }
        }
        Err(_) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::get_file_list;

    #[test]
    fn get_file_list_not_recursive_test() {
        let actual = get_file_list(".", false).unwrap().sort();
        let expect = vec![
            "./Cargo.toml",
            "./.gitignore",
            "./target",
            "./Cargo.lock",
            "./.git",
        ]
        .sort();

        assert_eq!(actual, expect);
    }

    #[test]
    fn get_file_list_recursive_test() {
        let actual = get_file_list("./test_resource", true).unwrap().sort();
        let expect = vec![
            "./test_resource/a.txt",
            "./test_resource/b.txt",
            "./test_resource/c/d.txt",
            "./test_resource/c/e.txt",
        ]
        .sort();

        assert_eq!(actual, expect);
    }
}
