use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::api::TemplateSource;

pub struct LocalSource {
    uri: String,
    target: PathBuf,
}

impl LocalSource {
    pub fn new(uri: String, target: PathBuf) -> Self {
        // todo: check if uri is a dir here
        Self { uri, target }
    }
}

impl TemplateSource for LocalSource {
    fn get_choices(&self) -> Vec<String> {
        let entries = fs::read_dir(self.uri.clone()).unwrap();
        let dirs = entries
            .map(|x| x.ok())
            .filter_map(|x| {
                if let Some(entry) = x {
                    if entry.metadata().unwrap().is_dir() {
                        return Some(entry.file_name().to_str().unwrap().to_string());
                    } else {
                        return None;
                    }
                } else {
                    return None;
                };
            })
            .collect();
        return dirs;
    }

    fn load_choice(&self, choice: String) -> bool {
        let path_to_clone = PathBuf::from(self.uri.clone()).join(choice.clone());
        copy_dir_all(path_to_clone, self.target.clone().join(choice.clone())).unwrap();
        return true;
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
