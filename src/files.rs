use std::{
  fs::{create_dir_all, hard_link, File},
  io::copy,
  path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::storage::join_home_path;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
  pub name: String,
  pub version: Option<String>,

  pub default_save_name: String,
  pub original_path: PathBuf,
  pub stored_path: Option<String>,
}

impl FileEntry {
  pub fn new(name: String, version: Option<String>) -> Self {
    FileEntry {
      name,
      version,

      ..Default::default()
    }
  }

  pub fn get_key(&self) -> String {
    if self.version.is_none() {
      return String::from(&self.name);
    }

    format!("{}-{}", self.name, self.version.as_ref().unwrap())
  }

  pub fn store(&mut self) -> Result<(), String> {
    let copied_to = self.copy(&join_home_path("files"))?;

    // set the stored path
    self.stored_path = copied_to.to_str().map(|s| s.to_string());

    Ok(())
  }

  pub fn copy(&self, to: &Path) -> Result<PathBuf, String> {
    // make the base directories
    create_dir_all(to).map_err(|_| "Failed to make the required parent folders")?;

    // add the file name to the destination path
    let to = to.join(self.get_key());

    // use the stored path if possible
    let copy_from = match self.stored_path.as_ref() {
      Some(path) => PathBuf::from(&path),
      None => self.original_path.clone(),
    };

    // open and copy the file to the storage
    let mut file = File::open(copy_from).map_err(|_| "Failed to open the file")?;
    let mut target = File::create(&to).map_err(|_| "Failed to create the output file")?;

    copy(&mut file, &mut target).map_err(|_| "Failed to copy the file to a stored location")?;

    Ok(to)
  }

  pub fn link(&self, to: &Path) -> Result<(), String> {
    // use the stored path if possible
    let from = match self.stored_path.as_ref() {
      Some(path) => PathBuf::from(&path),
      None => self.original_path.clone(),
    };

    hard_link(from, to).map_err(|_| "Failed to create a link to the file")?;

    Ok(())
  }
}
