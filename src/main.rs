extern crate atcoder_submissions_to_csv;

extern crate csv;
extern crate docopt;
#[macro_use]
extern crate serde_derive;

use atcoder_submissions_to_csv::*;
use docopt::Docopt;

const USAGE: &'static str = "
aoj_submissions_to_csv

Usage:
  aoj_submissions <contest-id> [--start=<start>] [--limit=<limit>]
  aoj_submissions (-h | --help)

Options:
  -h --help        Show this screen.
  --start=<start>  Index of the first submission.
  --limit=<limit>  Limit of the result.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_contest_id: String,
    flag_start: Option<u32>,
    flag_limit: Option<u32>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut writer = csv::Writer::from_writer(std::io::stdout());
    let submissions = fetch_submissions(&args.arg_contest_id, args.flag_start, args.flag_limit);
    for s in &submissions {
        writer.serialize(s).unwrap();
    }
    writer.flush().unwrap();
}
