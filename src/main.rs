mod cli;
mod parse;

use crate::cli::Opt;
use crate::parse::parse;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    if let Some(path) = opt.path.to_str() {
        let _ = parse(path, opt.json);
    }
}
