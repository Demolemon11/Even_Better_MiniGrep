use self::{arguments::Arguments, filecontent::FileContent};
pub mod arguments;
pub mod filecontent;
pub mod search;
pub struct Config {
    arguments: Arguments,
    filecontent: FileContent,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            arguments: Arguments::new(),
            filecontent: FileContent::new(),
        }
    }
}
