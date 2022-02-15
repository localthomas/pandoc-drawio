use anyhow::{Context, Result};

use std::path::{Path, PathBuf};

use crate::drawio::DrawioConverter;

/// A trait that abstracts over any file handler and allows to use multiple caching strategies.
pub trait ConverterCache {
    /// Convert the drawio file at `input_path` to the `format` and return the path to the output image.
    fn convert(&self, input_path: &Path, format: OutputFormat) -> Result<PathBuf>;
}

/// A simple enum listing all possible output formats in this program for drawio images.
pub enum OutputFormat {
    PDF,
    SVG,
}

/// A non-cache converter implementation that always converts the drawio files.
pub struct NoCacheConverter<'a> {
    converter: &'a DrawioConverter,
}

impl<'a> NoCacheConverter<'a> {
    /// Creates a new converter using the provided [`DrawioConverter`] as the internal conversion method.
    pub fn new(converter: &'a DrawioConverter) -> Self {
        Self { converter }
    }
}

impl<'a> ConverterCache for NoCacheConverter<'a> {
    fn convert(&self, input_path: &Path, format: OutputFormat) -> Result<PathBuf> {
        let output_path = match format {
            OutputFormat::PDF => input_path.with_extension("pdf"),
            OutputFormat::SVG => input_path.with_extension("svg"),
        };

        self.converter
            .convert_to(input_path, output_path.as_path(), format)
            .context(format!(
                "could not convert from {:?} to {:?}",
                input_path, output_path
            ))?;

        Ok(output_path)
    }
}
