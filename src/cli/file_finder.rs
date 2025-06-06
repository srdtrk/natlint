//! Defines file finding utilities for the CLI.

use eyre::OptionExt;
use glob::glob;
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Finds all files matching the glob pattern within the given root directory.
/// # Errors
/// Returns an error if the pattern is invalid or if there is an error searching for files.
/// # Returns
/// A vector of paths to the matching files.
pub fn find_matching_files(
    root: &str,
    includes: Vec<String>,
    excludes: Vec<String>,
) -> eyre::Result<HashSet<PathBuf>> {
    let root_path = Path::new(root);

    // Build a GlobSet for exclude patterns.
    let mut builder = GlobSetBuilder::new();
    for pattern in excludes {
        let exclude_path = root_path.join(&pattern);
        let exclude_pattern_str = exclude_path
            .to_str()
            .ok_or_eyre(format!("Invalid exclude pattern: {root}/{pattern}"))?;
        builder.add(Glob::new(exclude_pattern_str)?);
    }
    let exclude_set: GlobSet = builder.build()?;

    Ok(includes
        .into_iter()
        .map(|pattern| -> eyre::Result<_> {
            let include_path = root_path.join(&pattern);
            let include_path_str = include_path
                .to_str()
                .ok_or_eyre(format!("Invalid include pattern: {root}/{pattern}"))?;

            Ok(glob(include_path_str)?.collect::<Result<Vec<_>, _>>()?)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .filter(|entry| entry.is_file())
        .filter(|entry| !exclude_set.is_match(entry))
        .collect::<HashSet<_>>())
}
