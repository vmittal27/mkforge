pub use comrak::Arena;
/**
 * ============================================================================
 * Markdown Parser Module
 * Copyright (c) 2025 Viresh Mittal
 *
 * Parse Markdown files using the comrak library and return
 * an Abstract Syntax Tree (AST) for further processing.
 * ============================================================================
*/
use comrak::nodes::AstNode;
use comrak::{ComrakOptions, parse_document};
use std::fs;
use std::io::Error;

/// Supported Markdown flavors for parsing.
/// Currently only CommonMark and GitHub Flavored Markdown (GFM) are implemented.
///
/// This enum can be extended in the future to support more flavors.
///
/// Specifications for each flavor are pulled from here:
/// https://github.com/commonmark/commonmark-spec/wiki/markdown-flavors
pub enum Flavor {
    CommonMark,
    GitHub,
}

impl Flavor {
    /// Returns a string representation of the flavor.
    pub fn as_str(&self) -> &str {
        match self {
            Flavor::CommonMark => "CommonMark",
            Flavor::GitHub => "GitHub Flavored Markdown",
        }
    }

    /// Parses a string to return the corresponding Flavor enum.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "CommonMark" => Some(Flavor::CommonMark),
            "GitHub" => Some(Flavor::GitHub),
            _ => None,
        }
    }

    /// Converts the Flavor to ComrakOptions for parsing.
    pub fn to_options(&self) -> ComrakOptions<'static> {
        match self {
            Flavor::CommonMark => ComrakOptions::default(),

            // Github Flavored Markdown (GFM) options.
            // The options chosen here are based on the CLI code from comrak.
            // See https://github.com/kivikakk/comrak/blob/main/src/main.rs
            Flavor::GitHub => ComrakOptions {
                extension: comrak::ComrakExtensionOptions {
                    table: true,
                    strikethrough: true,
                    autolink: true,
                    tagfilter: true,
                    tasklist: true,
                    ..Default::default()
                },
                render: comrak::ComrakRenderOptions {
                    github_pre_lang: true,
                    gfm_quirks: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

///
/// Markdown Parser is a container for holding
/// the state needed for the parser,
/// including the file path and parse options.
pub struct ParseConfig {
    options: ComrakOptions<'static>,
    flavor: Flavor,
    file_path: String,
}

impl ParseConfig {
    pub fn new(file_path: impl Into<String>, flavor: Flavor) -> Self {
        let options = flavor.to_options();
        ParseConfig {
            options,
            flavor,
            file_path: file_path.into(),
        }
    }
}

/// Extracts the AST for a given parse configuration.
/// This function reads the file content,
/// parses it using the comrak library,
/// and returns the AST.
pub fn extract_ast<'a>(
    config: &ParseConfig,
    arena: &'a Arena<AstNode<'a>>,
) -> Result<&'a AstNode<'a>, Error> {
    // Read the file content
    let md = fs::read_to_string(&config.file_path)?;

    // Parse the document using comrak
    let ast = parse_document(arena, &md, &config.options);

    // Return the AST
    Ok(ast)
}
