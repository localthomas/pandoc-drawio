/*
SPDX-FileCopyrightText: 2022 localthomas
SPDX-License-Identifier: MIT OR Apache-2.0
*/

mod cache;
mod config;
mod drawio;
mod pandoc;

use std::{ffi::OsStr, io::Write, path::Path};

use anyhow::{anyhow, Context, Result};
use cache::{ConverterCache, NoCacheConverter, OutputFormat};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    config::Config,
    drawio::DrawioConverter,
    pandoc::{Image, PandocDocument},
};

fn main() -> Result<()> {
    // read the CLI arguments and options
    let config = Config::new().context("could not create configuration")?;
    // print credits and exit
    if config.credits {
        let credits = include_str!("../license.html");
        println!("{}", credits);
        return Ok(());
    }

    // prepare the drawio converter
    let drawio = DrawioConverter::new(&config.xvfb_run_cmd, &config.drawio_cmd)
        .context("could not create drawio converter")?;
    let converter = NoCacheConverter::new(&drawio);

    // read the pandoc AST from stdin
    let stdin = std::io::stdin();
    // Note: do not read all of the stdin data into a buffer, as the stdin input of pandoc can be very large:
    // e.g. `fill_buf()` would not work due to an internal size limitation of the buffer.
    let input_data = stdin.lock();

    // convert the AST to a Rust representation
    let mut pandoc = PandocDocument::new(input_data).context("could not create pandoc document")?;

    // get all possible images from the document and filter for *.drawio files
    let mut drawio_images: Vec<Image> = pandoc
        .get_all_images()
        .into_iter()
        .filter(|image| {
            let path_string: String = image.image_url_read_only().clone();
            Path::new(&path_string).extension().and_then(OsStr::to_str) == Some("drawio")
        })
        .collect();

    // convert each image to its output format
    drawio_images
        .par_iter_mut()
        .try_for_each(|image| -> Result<()> {
            convert_image(&converter, image, &config.format).context("could not convert image")?;
            Ok(())
        })?;

    // write the document AST back as JSON to stdout
    std::io::stdout()
        .write_all(
            &pandoc
                .to_json()
                .context("could not get JSON format of pandoc document")?,
        )
        .context("could not write the pandoc document to stdout")?;

    Ok(())
}

/// Converts the image to a suitable output format depending on the document output format.
/// On success, the `image` was altered to reference the new image file.
fn convert_image(converter: &dyn ConverterCache, image: &mut Image, format: &str) -> Result<()> {
    let path_string: String = image.image_url_read_only().clone();
    let input_path = Path::new(&path_string);

    let output_format = match format {
        "pdf" | "latex" | "context" => OutputFormat::Pdf,
        "html" | "html5" | "html4" => OutputFormat::Svg,
        _ => return Err(anyhow!("unknown or unsupported format: {}", format)),
    };

    let output_path = converter
        .convert(input_path, output_format)
        .context("could not convert to output format")?;

    // change the pandoc image path to the new output path
    let image_url_reference = image.image_url();
    *image_url_reference = output_path
        .to_str()
        .ok_or_else(|| {
            anyhow!(
                "the output path for a converted file is not valid utf8: {:?}",
                output_path
            )
        })?
        .to_string();

    Ok(())
}
