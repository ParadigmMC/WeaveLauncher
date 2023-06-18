use std::path::PathBuf;

pub trait PathBufExt {
    fn unwrap_to_string(&self) -> String;
}

impl PathBufExt for PathBuf {
    fn unwrap_to_string(&self) -> String {
        self.to_str().unwrap().to_owned()
    }
}