use std::{fs, path::Path, process::Command};

use anyhow::{anyhow, Context, Result};

use crate::cache::OutputFormat;

/// Holds all settings for converting a drawio file to other output formats.
pub struct DrawioConverter {
    drawio_path: String,
    xvfb_run_path: String,
}

impl DrawioConverter {
    /// Uses the provided command path and tries to access it.
    /// Spawns an `xvfb-run` environment.
    pub fn new(xvfb_run_path: &str, drawio_path: &str) -> Result<Self> {
        let mut xvfb_run = Command::new(xvfb_run_path);
        xvfb_run.output().context(format!(
            "could not execute '{}' during probing command. Is it installed?",
            xvfb_run_path
        ))?;

        let mut drawio_command = Command::new(drawio_path);
        // test an execution
        drawio_command.output().context(format!(
            "could not execute '{}' probing command. Is it installed?",
            drawio_path
        ))?;

        let mut command = xvfb_run;
        command.arg(drawio_path);
        Ok(Self {
            drawio_path: drawio_path.to_string(),
            xvfb_run_path: xvfb_run_path.to_string(),
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
            OutputFormat::PDF => "pdf",
            OutputFormat::SVG => "svg",
        };

        let output = Command::new(&self.xvfb_run_path)
            .arg(&self.drawio_path)
            .arg("--crop")
            .arg("-x")
            .arg("-o")
            .arg(output_path)
            .arg("-f")
            .arg(format_string)
            .arg(input_path)
            .arg("--no-sandbox")
            .arg("--disable-gpu")
            .arg("--disable-dev-shm-usage")
            .output()
            .context("could not execute conversion command")?;
        if !output_path.exists() {
            return Err(anyhow!(
                "output path ({:?}) was not created:\n{}",
                output_path,
                String::from_utf8(output.stdout).unwrap_or("--non-utf8 output--".to_string()),
            ));
        }
        Ok(())
    }
}
