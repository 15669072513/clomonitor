
use anyhow::{format_err, Result}; // 确保顶部已引入



use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};
use crate::linter::util::path;
use crate::linter::util::path::Globs;

/// Check identifier.
pub(crate) const ID: CheckId = "gitignore";

/// Check score weight.
pub(crate) const WEIGHT: usize = 2;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::AntIncubator];

/// Patterns used to locate a file in the repository.
pub(crate) static FILE_PATTERNS: [&str; 1] = [
    ".gitignore",
];

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference
    if let Some(path) = path::find(&Globs {
        root: &input.li.root,
        patterns: &FILE_PATTERNS,
        case_sensitive: false,
    })? {
        let abs_path = input.li.root.join(&path);

        // 打印调试信息
        // println!(
        //     "Found matching file at path: {}, patterns: {:?}，abs path: {}",
        //     path.display(),
        //     &FILE_PATTERNS,
        //     abs_path.display()
        // );


        // 检查文件是否存在且可读（双重验证）
        if !abs_path.exists() {
            anyhow::bail!("File disappeared after finding: {}", abs_path.display());
        }
        // 检查文件是否有内容（至少1行）
        let file_content = std::fs::read_to_string(&abs_path)
            .map_err(|e| format_err!("Failed to read file {}: {}", path.display(), e))?;

        if file_content.lines().count() >= 1 {
            return Ok(CheckOutput::passed().url(Some(path.display().to_string())));
        } else {
            // 文件为空
            // println!("File {} is empty", path.display());
        }
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;

}
