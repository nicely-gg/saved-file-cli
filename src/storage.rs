use std::{
  collections::HashMap,
  env,
  fs::{create_dir_all, File},
  path::{Path, PathBuf},
  sync::Mutex,
};

use lazy_static::lazy_static;

use crate::files::FileEntry;

const FILE_NAME: &str = "savedfile.json";
const HOME_FOLDER: &str = ".savedfile";

lazy_static! {
  static ref FILE_ENTRIES: Mutex<HashMap<String, FileEntry>> = Mutex::new(HashMap::new());
}

pub fn join_home_path(path: &str) -> PathBuf {
  let home = if cfg!(target_os = "windows") {
    env::var("USERPROFILE")
  } else {
    env::var("HOME")
  };

  // figure out the home path
  let binding = home.unwrap_or_default();
  let home_path = Path::new(binding.as_str());

  // join with the name of our home folder
  home_path.join(HOME_FOLDER).join(path)
}

pub fn get_all() -> Result<Vec<FileEntry>, String> {
  let entries = FILE_ENTRIES
    .lock()
    .map_err(|_| "Failed to lock FILE_ENTRIES")?;

  Ok(entries.values().cloned().collect())
}

pub fn add(entry: FileEntry) -> Result<(), String> {
  {
    let mut entries = FILE_ENTRIES
      .lock()
      .map_err(|_| "Failed to lock FILE_ENTRIES")?;

    entries.entry(entry.get_key()).or_insert(entry);
  }

  write()
}

pub fn remove(entry: FileEntry) -> Result<(), String> {
  {
    let mut entries = FILE_ENTRIES
      .lock()
      .map_err(|_| "Failed to lock FILE_ENTRIES")?;

    // remove the entry by its key
    entries.remove(&entry.get_key());
  }

  write()
}

pub fn find(entry: &FileEntry) -> Option<FileEntry> {
  let entries = match FILE_ENTRIES.lock() {
    Ok(entries) => entries,
    Err(_) => return None,
  };

  entries.get(&entry.get_key()).cloned()
}

pub fn read() -> Result<(), String> {
  let path = join_home_path(FILE_NAME);

  if !path.is_file() {
    return Ok(());
  }

  let file = File::open(path).map_err(|_| "Failed to open file")?;
  let content: HashMap<String, FileEntry> =
    serde_json::from_reader(file).map_err(|_| "Failed to parse JSON content")?;

  let mut entries = FILE_ENTRIES
    .lock()
    .map_err(|_| "Failed to lock FILE_ENTRIES")?;

  // assign the parsed content from the json file
  *entries = content;

  Ok(())
}

fn write() -> Result<(), String> {
  let entries = FILE_ENTRIES
    .lock()
    .map_err(|_| "Failed to lock FILE_ENTRIES")?;

  create_dir_all(join_home_path("")).map_err(|_| "Failed to create all parent directories")?;

  let file = File::create(join_home_path(FILE_NAME))
    .map_err(|err| format!("Failed to create or open file: {}", err))?;

  serde_json::to_writer_pretty(file, &(*entries))
    .map_err(|_| "Failed to write the JSON contents")?;

  Ok(())
}
