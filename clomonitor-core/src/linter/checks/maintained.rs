use anyhow::Result;

use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};

use super::datasource::scorecard;

/// Check identifier.
pub(crate) const ID: CheckId = "maintained";

/// Check score weight.
pub(crate) const WEIGHT: usize = 3;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Code];

/// Check main function.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    if input.li.mode == "local" {
        return Ok(CheckOutput::not_passed());
    }
    Ok(scorecard::get_check(&input.scorecard, ID).into())
}
