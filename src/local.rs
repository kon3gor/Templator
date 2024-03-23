use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::api::{TemplateSource, TemplatorResult};

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
    fn get_choices(&self) -> TemplatorResult<Vec<String>> {
        let entries = fs::read_dir(&self.uri)?;
        let dirs = entries
            .filter_map(|x| x.ok())
            .filter_map(|entry| {
                let ty = entry.file_type().ok()?;
                if ty.is_dir() {
                    return Some(entry.file_name().into_string().ok()?);
                } else {
                    return None;
                }
            })
            .collect();
        return Ok(dirs);
    }

    fn load_choice(&self, choice: String) -> TemplatorResult<()> {
        let path_to_clone = PathBuf::from(&self.uri).join(&choice);
        copy_dir_all(path_to_clone, self.target.join(&choice))?;
        return Ok(());
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
