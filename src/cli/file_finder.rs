//! Defines file finding utilities for the CLI.

use glob::glob;
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Finds all files matching the glob patterns.
/// # Errors
/// Returns an error if the pattern is invalid or if there is an error searching for files.
/// # Returns
/// A vector of paths to the matching files.
pub fn find_matching_files(
    root: &str,
    includes: Vec<String>,
    excludes: &[String],
) -> eyre::Result<HashSet<PathBuf>> {
    let root_path = Path::new(root);
    
    // If root path is not a directory, use its parent as root
    let (root_dir, default_pattern) = if !root_path.is_dir() && root_path.extension().is_some() {
        // The root is likely a file pattern like "/path/to/dir/**/*.sol"
        let parent = root_path.parent().unwrap_or_else(|| Path::new("."));
        let pattern = root_path.file_name().map_or_else(
            || "**/*.sol".to_string(),
            |n| n.to_string_lossy().to_string());
        (parent.to_path_buf(), Some(pattern))
    } else {
        (root_path.to_path_buf(), None)
    };

    // Build a GlobSet for exclude patterns.
    let mut builder = GlobSetBuilder::new();
    for pattern in excludes {
        builder.add(Glob::new(pattern)?);
    }
    let exclude_set: GlobSet = builder.build()?;
    
    // Process include patterns
    let mut matched_files = HashSet::new();
    
    // Use include patterns from arguments or default to "**/*.sol" if none provided
    let patterns_to_use = if !includes.is_empty() {
        includes
    } else if let Some(default) = default_pattern {
        vec![default]
    } else {
        vec!["**/*.sol".to_string()]
    };

    for pattern in patterns_to_use {
        // Check if pattern is absolute or relative
        let pattern_path = if Path::new(&pattern).is_absolute() {
            pattern.clone()
        } else {
            // Join the root directory with the pattern
            let joined = root_dir.join(&pattern);
            joined.to_string_lossy().to_string()
        };
        
        // Process glob pattern
        let entries = glob(&pattern_path)?
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .filter(|entry| entry.is_file())
            .filter(|entry| !exclude_set.is_match(entry))
            .collect::<Vec<_>>();
            
        matched_files.extend(entries);
    }

    Ok(matched_files)
}
