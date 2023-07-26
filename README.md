# SavedFile Utility

The `savedfile` utility is a CLI tool for managing files in a database.

## Installation

This project requires Rust to be installed. After ensuring that Rust is installed, you can install the project by running the appropriate script for your system:

On Unix-like systems:
```bash
./install.sh
```

On Windows:
```cmd
install.bat
```

## Usage

The `savedfile` utility is primarily used through three subcommands: `from`, `use`, and `remove`.

### From

The `from` subcommand is used to save a file to the database.

```bash
savedfile from --file <file> --named <name> [--version <version>]
```

- `--file`: The file to use locally. This argument is required.
- `--named`: What to name the provided file in the database. This argument is required.
- `--version`: The version of a file to use.

### Use

The `use` subcommand retrieves a file from the database.

```bash
savedfile use --name <name> [--save <local_name>] [--copy] [--version <version>]
```

- `--name`: The name of the file in the database. This argument is required.
- `--save`: What to save the file as locally.
- `--copy`: Copy the file instead of creating a link to it.
- `--version`: The version of a file to use.

### Remove

The `remove` subcommand removes a file from the database.

```bash
savedfile remove --name <name> [--version <version>]
```

- `--name`: The name of the file in the database. This argument is required.
- `--version`: The version of a file to use.

### List

The `list` subcommand lists all files or specific file if a name is provided in the database.

```bash
savedfile list [--name <name>]
```

- `--name`: The name of the file in the database.
