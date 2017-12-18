/*
 * UpdateHub
 * Copyright (C) 2017
 * O.S. Systems Sofware LTDA: contact@ossystems.com.br
 *
 * SPDX-License-Identifier:     GPL-2.0
 */

#[macro_use]
extern crate log;
extern crate stderrlog;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_ini;

extern crate parse_duration;

#[cfg(test)]
extern crate mktemp;

mod de_helpers;
mod se_helpers;

mod settings;
mod runtime_settings;
mod cmdline;

use cmdline::CmdLine;
use runtime_settings::RuntimeSettings;
use settings::Settings;

fn main() {
    let cmdline = CmdLine::parse_args();

    stderrlog::new().quiet(cmdline.quiet)
                    .verbosity(if cmdline.debug { 3 } else { 2 })
                    .init()
                    .expect("Failed to initialize the logger.");

    let settings = Settings::new().load().expect("Failed to load settings.");
    let _runtime_settings = RuntimeSettings::new().load(&settings.storage.runtime_settings)
                                                  .expect("Failed to load runtime settings.");
}
