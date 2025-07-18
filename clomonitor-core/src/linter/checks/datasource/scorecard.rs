use anyhow::{format_err, Error, Result};
use serde::Deserialize;
use tokio::process::Command;

use crate::linter::checks::CHECKS;

/// Scorecard report (list of checks).
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Scorecard {
    checks: Vec<ScorecardCheck>,
}

/// Scorecard check details.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct ScorecardCheck {
    pub name: String,
    pub reason: String,
    pub details: Option<Vec<String>>,
    pub score: f64,
    pub documentation: ScorecardCheckDocs,
}
// 实现 Default trait
impl Default for Scorecard {
    fn default() -> Self {
        Scorecard {
            checks: Vec::new(), // 默认空检查列表
        }
    }
}

/// Scorecard check documentation.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct ScorecardCheckDocs {
    pub url: String,
}

/// Get repository's OpenSSF Scorecard.
pub(crate) async fn scorecard(repo_url: &str, github_token: &str) -> Result<Scorecard> {
    let output = Command::new("scorecard")
        .env("GITHUB_TOKEN", github_token)
        .env_remove("GITHUB_REF")
        .arg(format!("--repo={repo_url}"))
        .arg("--format=json")
        .arg("--show-details")
        .arg("--checks=Binary-Artifacts,Code-Review,Dangerous-Workflow,Dependency-Update-Tool,Maintained,Signed-Releases,Token-Permissions")
        .output()
        .await?;
    if !output.status.success() {
        return Err(format_err!("{}", String::from_utf8_lossy(&output.stderr)));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let scorecard: Scorecard = serde_json::from_str(stdout.as_ref())?;
    Ok(scorecard)
}

// Get a check from the scorecard provided if available.
pub(crate) fn get_check<'a>(
    scorecard: &'a Result<Scorecard>,
    check_id: &'a str,
) -> Result<Option<&'a ScorecardCheck>, &'a Error> {
    match scorecard {
        Ok(scorecard) => Ok(scorecard
            .checks
            .iter()
            .find(|c| &c.name == CHECKS[check_id].scorecard_name.as_ref().unwrap())),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use crate::linter::checks::code_review;

    use super::*;

    #[test]
    fn get_check_found() {
        let scorecard = Ok(Scorecard {
            checks: vec![ScorecardCheck {
                name: "Code-Review".to_string(),
                reason: "test".to_string(),
                details: None,
                score: 8.0,
                documentation: ScorecardCheckDocs {
                    url: "https://test.url".to_string(),
                },
            }],
        });

        assert_eq!(
            get_check(&scorecard, code_review::ID).unwrap().unwrap(),
            &scorecard.as_ref().unwrap().checks[0]
        );
    }

    #[test]
    fn get_check_not_found() {
        let scorecard = Ok(Scorecard { checks: vec![] });

        assert!(get_check(&scorecard, code_review::ID).unwrap().is_none());
    }
}
