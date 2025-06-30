
use std::error::Error;
use anyhow::{Context, Result};
use std::fs;


use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};

/// Check identifier.
pub(crate) const ID: CheckId = "issue_template";

/// Check score weight.
pub(crate) const WEIGHT: usize = 2;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::AntIncubator];

pub(crate) const ISSUE_TEMPLATE_FILE: &str = ".github/ISSUE_TEMPLATE.md";
pub(crate) const ISSUE_TEMPLATE_DIR: &str = ".github/ISSUE_TEMPLATE";



/// Check main function for issue templates
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // 构建完整路径
    let issue_template_paths = [
        input.li.root.join(ISSUE_TEMPLATE_FILE),
        input.li.root.join(ISSUE_TEMPLATE_DIR),
    ];

    for path in &issue_template_paths {
        // 检查文件或目录是否存在
        if !path.exists() {
            continue;
        }

        println!("Found issue template at: {}", path.display());

        if path.is_file() {
            // 如果是文件，检查是否有内容
            let content = fs::read_to_string(path)
                .with_context(|| format!("Failed to read issue template file: {}", path.display()))?;

            if !content.trim().is_empty() {
                return Ok(CheckOutput::passed());
            }
        } else if path.is_dir() {
            // 如果是目录，检查其中是否有.md文件
            let has_md_files = fs::read_dir(path)
                .with_context(|| format!("Failed to read issue template directory: {}", path.display()))?
                .filter_map(|entry| entry.ok())
                .any(|entry| {
                    let entry_path = entry.path();
                    entry_path.is_file() &&
                        entry_path.extension().map_or(false, |ext| ext == "md")
                });

            if has_md_files {
                return Ok(CheckOutput::passed());
            }
        }
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {

}
