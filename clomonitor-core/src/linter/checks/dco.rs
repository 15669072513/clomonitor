use std::{path::Path, sync::LazyLock};

use anyhow::Result;
use regex::{Regex, RegexSet};

use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};

use super::datasource::github;

/// Check identifier.
pub(crate) const ID: CheckId = "dco";

/// Check score weight.
pub(crate) const WEIGHT: usize = 1;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 2] = [CheckSet::Code, CheckSet::CodeLite];

/// Maximum number of commits used to check if the repository requires DCO.
const DCO_MAX_COMMITS: usize = 20;

static CHECK_REF: LazyLock<RegexSet> =
    LazyLock::new(|| RegexSet::new([r"(?i)dco"]).expect("exprs in CHECK_REF to be valid"));

/// Check main function.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // DCO signature in commits
    if let Ok(passed) = commits_have_dco_signature(&input.li.root) {
        if passed {
            return Ok(CheckOutput::passed());
        }
    }
    if input.li.mode == "local" {
        return Ok(CheckOutput::not_passed());
    }

    // DCO check in Github
    if github::has_check(&input.gh_md, &CHECK_REF) {
        return Ok(CheckOutput::passed());
    }

    Ok(CheckOutput::not_passed())
}

/// Check if the last commits on the git repository located in the path
/// provided have the DCO signature.
fn commits_have_dco_signature(path: &Path) -> Result<bool, git2::Error> {
    static MERGE_PR_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^Merge pull request ").expect("valid expression"));
    static MERGE_BRANCH_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^Merge branch ").expect("valid expression"));
    static DCO_SIGNATURE_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?m)^Signed-off-by: ").expect("valid expression"));

    let repo = git2::Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let (mut processed, mut signed_off, mut merge) = (0, 0, 0);
    for oid in revwalk.take(DCO_MAX_COMMITS) {
        if oid.is_err() {
            continue;
        }
        let commit = repo.find_commit(oid.expect("checked if is an error above"))?;
        processed += 1;
        if let Some(msg) = commit.message() {
            if MERGE_PR_RE.is_match(msg) || MERGE_BRANCH_RE.is_match(msg) {
                merge += 1;
                continue;
            }
            if DCO_SIGNATURE_RE.is_match(msg) {
                signed_off += 1;
            } else {
                return Ok(false);
            }
        }
    }

    if signed_off == processed - merge {
        return Ok(true);
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_ref_match() {
        assert!(CHECK_REF.is_match(r"DCO"));
    }
}
