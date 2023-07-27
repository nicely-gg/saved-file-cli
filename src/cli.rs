use std::{
  collections::{HashMap, HashSet},
  fs::remove_file,
  path::{Path, PathBuf},
};

use clap::{value_parser, Arg, Command};
use console::Style;
use lazy_static::lazy_static;

use crate::{files::FileEntry, storage};

lazy_static! {
  static ref COMMAND: Command = Command::new("savedfile")
    .subcommand(
      Command::new("from")
        .about("Create a saved file from a provided local file")
        .arg(
          Arg::new("file")
            .help("The file to use locally")
            .required(true)
            .value_parser(value_parser!(String)),
        )
        .arg(
          Arg::new("named")
            .short('n')
            .long("named")
            .help("What to name the provided file in the database")
            .required(true)
            .value_parser(value_parser!(String)),
        )
        .arg(
          Arg::new("version")
            .short('v')
            .long("version")
            .help("The version of a file to use")
            .value_parser(value_parser!(String)),
        ),
    )
    .subcommand(
      Command::new("use")
        .about("Use a saved file by whatever you --named it using the from command")
        .arg(
          Arg::new("name")
            .help("The name of the file in the database")
            .required(true)
            .value_parser(value_parser!(String)),
        )
        .arg(
          Arg::new("output")
            .short('o')
            .long("output")
            .help("What to save the file as locally")
            .value_parser(value_parser!(String)),
        )
        .arg(
          Arg::new("copy")
            .short('c')
            .long("copy")
            .help("Copy the file instead of creating a link to it")
            .value_parser(value_parser!(bool)),
        )
        .arg(
          Arg::new("version")
            .short('v')
            .long("version")
            .help("The version of a file to use")
            .value_parser(value_parser!(String)),
        ),
    )
    .subcommand(
      Command::new("remove")
        .about("Remove a saved file")
        .arg(
          Arg::new("name")
            .help("The name of the file in the database")
            .required(true)
            .value_parser(value_parser!(String)),
        )
        .arg(
          Arg::new("version")
            .short('v')
            .long("version")
            .help("The version of a file to use")
            .value_parser(value_parser!(String)),
        ),
    )
    .subcommand(
      Command::new("list")
        .about(
          "Gives a list of all (or a specific) saved file(s) and their versions (a plus means there's a default version)"
        )
        .arg(
          Arg::new("name")
            .help("The name of the file in the database")
            .required(false)
            .value_parser(value_parser!(String)),
        )
    );
}

pub fn handle() {
  let matches = COMMAND.clone().get_matches();

  let result = match matches.subcommand() {
    Some(("from", matches)) => handle_from(
      matches.get_one("file"),
      matches.get_one("named"),
      matches.get_one("version"),
    ),

    Some(("use", matches)) => handle_use(
      matches.get_one("name"),
      matches.get_one("version"),
      matches.get_one("output"),
      matches.get_one("copy"),
    ),

    Some(("remove", matches)) => handle_remove(matches.get_one("name"), matches.get_one("version")),
    Some(("list", matches)) => handle_list(matches.get_one("name")),

    _ => Err(format!(
      "Couldn't find a subcommand for that. Try using --help.",
    )),
  };

  if let Err(e) = result {
    eprintln!(
      "{} {e}",
      Style::new().red().bright().bold().apply_to("Error:"),
    );
  }
}

fn handle_from(
  file_name: Option<&String>,
  name: Option<&String>,
  version: Option<&String>,
) -> Result<(), String> {
  // make sure the required arguments are provided
  if file_name.is_none() || name.is_none() {
    return Err(String::from("Missing required arguments"));
  }

  let file_name = file_name.unwrap();
  let name = name.cloned().unwrap();

  if name.contains('@') {
    return Err(String::from("The name cannot contain an '@' symbol"));
  }

  // make sure the file exists
  let full_path = Path::new(file_name);
  if !full_path.is_file() {
    return Err(format!("File {} does not exist", file_name));
  }

  let mut entry = FileEntry::new(name, version.cloned());

  // set the default save name to the name of the provided file
  entry.original_path = full_path
    .canonicalize()
    .map_err(|_| String::from("Failed to canonicalize the path"))?
    .to_path_buf();

  entry.default_save_name = full_path
    .file_name()
    .map(|os_str| os_str.to_str().unwrap_or("savedfile"))
    .unwrap_or("savedfile")
    .to_string();

  entry.store()?;

  let do_link_original = dialoguer::Select::new()
    .with_prompt("Replace original with a link to the stored one?")
    .default(0)
    .items(&["Yes", "No"])
    .interact()
    .unwrap_or(1);

  if do_link_original == 0 {
    remove_file(full_path).map_err(|_| "Failed to remove the original file")?;
    entry.link(full_path)?;
  }

  storage::add(entry)?;

  Ok(())
}

fn handle_use(
  name: Option<&String>,
  version: Option<&String>,
  save_as: Option<&String>,
  do_copy: Option<&bool>,
) -> Result<(), String> {
  if name.is_none() {
    return Err(String::from("Please specify the name of the file you want"));
  }

  let entry = match storage::find(&FileEntry::new(name.cloned().unwrap(), version.cloned())) {
    Some(entry) => entry,
    None => {
      println!("No entry found with that name and version");
      return Ok(());
    },
  };

  let save_path = PathBuf::from(save_as.unwrap_or(&entry.default_save_name));

  if let Some(&true) = do_copy {
    entry.copy(&save_path)?;
    return Ok(());
  };

  match entry.link(&save_path) {
    Ok(_) => (),
    Err(_) => {
      println!("Failed to create a link, making copy instead");
      entry.copy(&save_path)?;
    },
  }

  Ok(())
}

fn handle_remove(name: Option<&String>, version: Option<&String>) -> Result<(), String> {
  if name.is_none() {
    return Err(String::from("Please specify the name of the item you want"));
  }

  let entry = match storage::find(&FileEntry::new(name.cloned().unwrap(), version.cloned())) {
    Some(entry) => entry,
    None => {
      println!("No entry found with that name and version");
      return Ok(());
    },
  };

  storage::remove(entry)?;

  Ok(())
}

fn handle_list(name: Option<&String>) -> Result<(), String> {
  let entries = storage::get_all()?;
  let mut files: HashMap<String, HashSet<Option<String>>> = HashMap::new();

  let do_search = name.is_some();
  let search_name = name.cloned().unwrap_or_default();

  for entry in entries {
    if do_search && entry.name != search_name {
      continue;
    }

    files
      .entry(entry.name.clone())
      .or_insert_with(HashSet::new)
      .insert(entry.version.clone());
  }

  for (entry_name, versions) in files {
    let mut has_default = false;
    let versions = versions
      .iter()
      .filter_map(|version| match version {
        Some(version) => Some(String::from(version)),
        _ => {
          has_default = true;
          None
        },
      })
      .collect::<Vec<String>>()
      .join(", ");

    print!("{}", Style::new().bold().apply_to(entry_name));

    // check if there is an empty string
    if has_default {
      print!("{}", Style::new().green().bright().apply_to("+"));
    }

    if versions.len() > 0 {
      println!(
        "{} {}",
        Style::new().black().bright().apply_to(" versions:"),
        Style::new().cyan().bright().apply_to(versions),
      );
    }
  }

  Ok(())
}
