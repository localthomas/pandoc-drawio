/*
SPDX-FileCopyrightText: 2021 localthomas
SPDX-License-Identifier: MIT OR Apache-2.0
*/

use anyhow::Result;
use clap::{App, Arg};

/// Holds the static configuration for the program.
/// Can be used to alter the behavior of the execution or to configure notification provider.
#[derive(Debug)]
pub struct Config {
    pub xvfb_run_cmd: Option<String>,
    pub drawio_cmd: String,
    pub credits: bool,
    pub format: String,
}

impl Config {
    /// Read command line arguments and flags, as well as environment variables to parse the runtime configuration.
    pub fn new() -> Result<Self> {
        const CREDITS: (&str, &str) = (
            "credits",
            "if set, print the licensing information as HTML and exit",
        );
        const XVFB_RUN_CMD: (&str, &str) = (
            "xvfb-run-cmd",
            "the path to the xvfb-run executable, required if no x server is available (e.g. if running headless) (optional)",
        );
        const DRAWIO_CMD: (&str, &str, &str) =
            ("drawio-cmd", "the path to the drawio executable", "drawio");
        const FORMAT: (&str, &str) = (
            "format",
            "the format string for the output format of pandoc (e.g. html5)",
        );

        let matches = App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg(
                Arg::new(CREDITS.0)
                    .long(CREDITS.0)
                    .help(CREDITS.1)
                    .takes_value(false),
            )
            .arg(
                Arg::new(XVFB_RUN_CMD.0)
                    .long(XVFB_RUN_CMD.0)
                    .help(XVFB_RUN_CMD.1)
                    .takes_value(true),
            )
            .arg(
                Arg::new(DRAWIO_CMD.0)
                    .long(DRAWIO_CMD.0)
                    .help(DRAWIO_CMD.1)
                    .default_value(DRAWIO_CMD.2)
                    .takes_value(true),
            )
            .arg(Arg::new(FORMAT.0).help(FORMAT.1).required(true))
            .get_matches();

        Ok(Self {
            credits: matches.is_present(CREDITS.0),
            xvfb_run_cmd: matches
                .value_of(XVFB_RUN_CMD.0)
                .map(|value| value.to_string()),
            drawio_cmd: matches.value_of(DRAWIO_CMD.0).unwrap().to_string(),
            format: matches.value_of(FORMAT.0).unwrap().to_string(),
        })
    }
}
