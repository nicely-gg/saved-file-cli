mod cli;
mod files;
mod storage;

fn main() {
  match storage::read() {
    Ok(_) => (),
    Err(err) => eprintln!("Something went wrong reading the storage file: {err}"),
  }

  cli::handle();
}
