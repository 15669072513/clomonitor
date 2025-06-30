use std::sync::LazyLock;

use anyhow::Result;
use regex::RegexSet;

use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};

use super::util::helpers::find_file_or_readme_ref;

/// Check identifier.
pub(crate) const ID: CheckId = "get_started";

/// Check score weight.
pub(crate) const WEIGHT: usize = 2;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::AntIncubator];

/// Patterns used to locate a file in the repository.
pub(crate) static FILE_PATTERNS: [&str; 2] = [
    "docs/quickstart*",
    "docs/get*started*"
];

static README_REF: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        r"(?im)^#+.*get.started.*$",
        r"(?im)^#+.*快速开始.*$",
        r"(?im)^#+.*quickstart.*$",

        r"(?im)^get.started$",
        r"(?im)^快速开始$",
        r"(?im)^quickstart$",

        r"(?i)\[.*get.started.*\]\(.*\)",
        r"(?i)\[.*quickstart.*\]\(.*\)",
        r"(?i)\[.*快速开始.*\]\(.*\)",
        r"(?im)^#+.*(开始使用|入门指南).*$"  // 添加更多中文关键词

    ])
    .expect("exprs in README_REF to be valid")
});

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference in README file
    let output = find_file_or_readme_ref(input, &FILE_PATTERNS, &README_REF)?;
    if output.passed {
        return Ok(output);
    }
    if input.li.mode == "local" {
        return Ok(CheckOutput::not_passed());
    }
    // File in Github (default community health file, for example)
    if let Some(coc) = &input.gh_md.code_of_conduct {
        if coc.url.is_some() {
            return Ok(CheckOutput::passed().url(coc.url.clone()));
        }
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_ref_match() {
        assert!(README_REF.is_match("## Quickstarts"));
        assert!(README_REF.is_match(
            r"
...

## Quickstarts

This project follows the [CNCF Code of Conduct](https://github.com/cncf/foundation/blob/master/code-of-conduct.md).
...
            "
        ));
        assert!(README_REF.is_match(
            r"
...
快速开始
get started

---------------
...
            "
        ));
        assert!(README_REF.is_match("[get started](...)"));
    }
}
