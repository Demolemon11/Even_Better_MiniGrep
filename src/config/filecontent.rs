use super::arguments::Arguments;
use std::fs;

pub struct FileContent(pub Vec<String>);
impl FileContent {
    pub fn new() -> Self {
        Self(
            fs::read_to_string(&Arguments::new().path)
                .unwrap_or_else(|err| {
                    eprintln!("There is an error: Probabely the file does not exit Or you input a wrong file path.");
                    err.to_string()
                })
                .lines()
                .map(|item| item.to_string())
                .collect(),
        )
    }
}
