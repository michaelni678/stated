use itertools::Itertools;
use syn::{Attribute, Meta, Result};

use crate::utilities::{squote::parse_squote, stateset::Stateset};

/// Handles documentation generation.
#[derive(Default)]
pub struct Documentation {
    pub description: bool,
    pub ugly: bool,
}

impl Documentation {
    /// Configures the documentation with the given metas. Skips metas without
    /// the `docs` attribute.
    pub fn configure_with_metas<'a, M>(&mut self, metas: M) -> Result<()>
    where
        M: IntoIterator<Item = &'a Meta>,
    {
        let metas = metas
            .into_iter()
            .filter(|meta| meta.path().is_ident("docs"));

        for meta in metas {
            meta.require_list()?.parse_nested_meta(|meta| {
                if meta.path.is_ident("description") {
                    if self.description {
                        return Err(meta.error("redundant `description` attribute"));
                    }

                    self.description = true;
                } else if meta.path.is_ident("ugly") {
                    if self.ugly {
                        return Err(meta.error("redundant `ugly` attribute"));
                    }

                    self.ugly = true;
                } else {
                    return Err(meta.error("invalid attribute"));
                }

                Ok(())
            })?;
        }

        Ok(())
    }
}

/// Handles generating a description.
pub struct Description<'a> {
    stateset: &'a Stateset,
    lines: Vec<DescriptionLine>,
}

impl<'a> Description<'a> {
    /// Create a new description
    pub fn new(stateset: &'a Stateset) -> Self {
        Self {
            stateset,
            lines: Vec::new(),
        }
    }

    /// Add a line to the description builder.
    pub fn line(mut self, line: DescriptionLine) -> Self {
        self.lines.push(line);
        self
    }

    /// Generates the description. Panics if a required state attribute isn't
    /// found.
    pub fn generate(self) -> Attribute {
        // All descriptions start with a blank HTML comment. Rustdoc generates
        // a summary line for modules (https://github.com/rust-lang/rust/blob/eed187cfce988dd669b7d9161f324433e64358ee/src/librustdoc/html/render/print_item.rs#L500).
        // The descriptions Stated generates should not be shown as a summary.
        // This HTML comment tricks rustdoc so that if there is no
        // documentation above the current, the summary line will be blank.
        let mut description = String::from("<!-- -->");

        for line in self.lines {
            let states = self.stateset.get(&line.attribute).expect("invalid attribute");

            let label = line.label.unwrap_or(line.attribute);
            let states = states.iter().map(|state| state.to_string()).join(", ");

            // Don't write this line if there are no states to list.
            if states.is_empty() {
                continue;
            }

            // NOTE: There are two spaces between the description and the newline for a markdown soft line break.
            description = format!("{description}  \n**{label}**: {states}");
        }

        parse_squote!(#[doc = #description])
    }
}

/// A description line.
pub struct DescriptionLine {
    attribute: String,
    label: Option<String>,
}

impl DescriptionLine {
    /// Create a new line.
    pub fn new(attribute: impl Into<String>) -> Self {
        Self {
            attribute: attribute.into(),
            label: None,
        }
    }

    /// Set the label of the line.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label.replace(label.into());
        self
    }
}
