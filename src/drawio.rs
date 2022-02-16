/*
SPDX-FileCopyrightText: 2022 localthomas
SPDX-License-Identifier: MIT OR Apache-2.0
*/

use std::{fs, path::Path, process::Command};

use anyhow::{anyhow, Context, Result};

use crate::cache::OutputFormat;

/// Holds all settings for converting a drawio file to other output formats.
pub struct DrawioConverter {
    drawio_path: String,
    xvfb_run_path: Option<String>,
}

impl DrawioConverter {
    /// Uses the provided command path and tries to access it.
    /// Spawns an `xvfb-run` environment.
    pub fn new(xvfb_run_path: &Option<String>, drawio_path: &str) -> Result<Self> {
        // test execution with xvfb-run, but only if available
        if let Some(xvfb_run_path) = xvfb_run_path {
            let mut xvfb_run = Command::new(xvfb_run_path);
            xvfb_run.arg("--help");
            xvfb_run.output().context(format!(
                "could not execute '{}' during probing command. Is it installed?",
                xvfb_run_path
            ))?;
        }

        let mut drawio_command = Command::new(drawio_path);
        drawio_command.arg("--help");
        // test an execution
        drawio_command.output().context(format!(
            "could not execute '{}' probing command. Is it installed?",
            drawio_path
        ))?;

        Ok(Self {
            drawio_path: drawio_path.to_string(),
            xvfb_run_path: xvfb_run_path.clone(),
        })
    }

    /// Executes a command to produce PDF output to the output path.
    pub fn convert_to(
        &self,
        input_path: &Path,
        output_path: &Path,
        format: OutputFormat,
    ) -> Result<()> {
        if !input_path.exists() {
            return Err(anyhow!("input path does not exist: {:?}", input_path));
        }
        // remove a previous output file, if present
        if output_path.exists() {
            fs::remove_file(output_path)
                .context("could not remove output file prior to conversion")?;
        }

        let format_string = match format {
            OutputFormat::Pdf => "pdf",
            OutputFormat::Svg => "svg",
        };

        let mut start_command = if let Some(xvfb_run_path) = &self.xvfb_run_path {
            let mut cmd = Command::new(xvfb_run_path);
            cmd.arg(&self.drawio_path);
            cmd
        } else {
            Command::new(&self.drawio_path)
        };

        start_command
            .arg("--crop")
            .arg("-x")
            .arg("-o")
            .arg(output_path)
            .arg("-f")
            .arg(format_string)
            .arg(input_path)
            .arg("--no-sandbox")
            .arg("--disable-gpu")
            .arg("--disable-dev-shm-usage");
        let output = start_command
            .output()
            .context("could not execute conversion command")?;

        // check for errors during or after execution
        // error code 133 seems to indicate that no graphic user interface is available and/or xvfb-run did not work
        match output.status.code() {
            Some(133) | None => {
                return Err(anyhow!("running conversion command exited with error: is an x server available (or xvfb-run)?"));
            }
            _ => (),
        }

        // unfortunately, drawio does not exit with an error, if the drawio image was not converted
        if !output_path.exists() {
            return Err(anyhow!(
                "output path ({:?}) was not created:\n{}",
                output_path,
                String::from_utf8(output.stdout)
                    .unwrap_or_else(|_| "--non-utf8 output--".to_string()),
            ));
        }
        Ok(())
    }
}
