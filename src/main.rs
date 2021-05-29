// Copyright 2021 The Spyder Authors.
// Use of this source code is governed by the MIT License which can be
// found in the LICENSE file.

use spyder::run_file;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,
    #[structopt(short, long)]
    pub debug: bool,
}

fn main() {
    let args = Cli::from_args();
    let result = run_file(&args.path, args.debug);
    println!("{:?}", result);
}
