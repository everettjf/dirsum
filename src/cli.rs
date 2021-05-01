use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "dirsum",
  about = "Get formatted summary for specified directory.",
)]
pub struct Opt {
  #[structopt(short, long, parse(from_os_str), help("Path to a directory"))]
  pub path: PathBuf,

  #[structopt(short, long, help("Print with json format"))]
  pub json: bool,
}
