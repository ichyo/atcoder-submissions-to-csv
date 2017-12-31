extern crate atcoder_submissions_to_csv;

use atcoder_submissions_to_csv::*;

fn main() {
    println!("{:?}", fetch_submissions("arc088", Some(0), Some(1)))
}
