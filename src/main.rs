// Copyright (C) 2017, 2018 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: Apache-2.0

use log::{error, info};
use stderrlog;
use structopt::StructOpt;
use updatehub;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "updatehub",
    author = "O.S. Systems Software LTDA. <contact@ossystems.com.br>",
    about = "A generic and safe Firmware Over-The-Air agent."
)]
#[structopt(raw(version = "updatehub::version()"))]
struct Opt {
    /// Increase the verboseness level
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
}

fn run() -> Result<(), failure::Error> {
    let opt = Opt::from_args();

    stderrlog::new().verbosity(opt.verbose + 1).init()?;

    info!("Starting UpdateHub Agent {}", updatehub::version());

    let settings = updatehub::Settings::load()?;
    updatehub::run(settings)?;

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        error!("{}", e);
        e.iter_causes()
            .skip(1)
            .for_each(|e| error!(" caused by: {}\n", e));

        std::process::exit(1);
    }
}
