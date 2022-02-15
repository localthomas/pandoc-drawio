/*
SPDX-FileCopyrightText: 2022 localthomas
SPDX-License-Identifier: MIT OR Apache-2.0
*/

use std::io::Read;

use anyhow::{Context, Result};
use pandoc_types::definition::{Inline, IterBlocks, IterInlines, Pandoc, Target};

/// Holds the pandoc document representation and allows iterating over items.
#[derive(Debug)]
pub struct PandocDocument {
    document: Pandoc,
}

impl PandocDocument {
    /// Attempts to read a pandoc JSON abstract syntax tree from stdin.
    /// Note that no other stdin should be open or opened before using this function.
    pub fn new<R: Read>(data_source: R) -> Result<Self> {
        let document: Pandoc = serde_json::from_reader(data_source)
            .context("could not deserialize pandoc JSON data from stdin")?;

        Ok(Self { document })
    }

    /// Creates a list with mutable references to images in the document.
    pub fn get_all_images(&mut self) -> Vec<Image> {
        self.document
            .iter_blocks_mut()
            .map::<Vec<Image>, _>(|block| {
                block
                    .iter_inlines_mut()
                    .filter_map(|inline| match inline {
                        Inline::Image(_, _, target) => Some(Image {
                            pandoc_image_target: target,
                        }),
                        _ => None,
                    })
                    .collect()
            })
            .flatten()
            .collect()
    }

    /// Converts the pandoc document to JSON AST as string.
    pub fn to_json(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(&self.document).context("could not serialize the pandoc document")
    }
}

/// Represents an image within a [`PandocDocument`], therefore their lifetimes are coupled.
#[derive(Debug)]
pub struct Image<'a> {
    pandoc_image_target: &'a mut Target,
}

impl Image<'_> {
    /// Returns a mutable reference to the image URL as [`String`].
    pub fn image_url(&mut self) -> &mut String {
        &mut self.pandoc_image_target.url
    }

    /// Returns a read-only reference to the image URL.
    pub fn image_url_read_only(&self) -> &String {
        &self.pandoc_image_target.url
    }
}
