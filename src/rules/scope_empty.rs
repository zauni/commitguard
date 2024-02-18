use crate::parser::Commit;

use super::{Condition, NoOpts, Rule, Severity};
use miette::{miette, LabeledSpan, Report};

pub struct ScopeEmptyRule {
    pub opts: NoOpts,
}

impl Rule for ScopeEmptyRule {
    fn run(&self, commit: &Commit) -> Option<Report> {
        let severity = &self.opts.0;
        let condition = &self.opts.1;

        if severity == &Severity::Off {
            return None;
        }

        let is_valid = match condition {
            Condition::Never => commit.scope.is_some(),
            Condition::Always => commit.scope.is_none(),
        };

        let mut labels = None;

        if let Some(scope) = &commit.scope {
            labels = Some(vec![LabeledSpan::at(
                scope.start()..scope.end(),
                "not allowed scope",
            )]);
        }

        if !is_valid {
            return Some(
                miette!(
                    severity = match severity {
                        Severity::Warning => miette::Severity::Warning,
                        Severity::Error => miette::Severity::Error,
                        Severity::Off => miette::Severity::Advice,
                    },
                    labels = labels.unwrap_or_default(),
                    help = String::from("scope")
                        + match condition {
                            Condition::Never => " may not be empty",
                            Condition::Always => " must be empty",
                        },
                    code = "rule/scope-empty",
                    url = "https://example.com",
                    "Scope",
                )
                .with_source_code(commit.raw.clone()),
            );
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_scope() {
        let mut commit: Commit<'_> = Default::default();
        commit.scope = None;

        // If the condition is `Never` and the scope is empty, the rule should return an error (read as "the scope should never be empty")
        let rule = ScopeEmptyRule {
            opts: NoOpts(Severity::Error, Condition::Never),
        };

        assert!(rule.run(&commit).is_some());

        // If the condition is `Always` and the scope is empty, the rule should return `None` (read as "the scope should always be empty")
        let rule = ScopeEmptyRule {
            opts: NoOpts(Severity::Error, Condition::Always),
        };

        assert!(rule.run(&commit).is_none());
    }

    #[test]
    fn test_filled_scope() {
        let mut commit: Commit<'_> = Default::default();
        commit.scope = Some(Default::default());

        // If the condition is `Never` and the scope is filled, the rule should return `None` (read as "the scope should never be empty")
        let rule = ScopeEmptyRule {
            opts: NoOpts(Severity::Error, Condition::Never),
        };

        assert!(rule.run(&commit).is_none());

        // If the condition is `Always` and the scope is filled, the rule should return an error (read as "the scope should always be empty")
        let rule = ScopeEmptyRule {
            opts: NoOpts(Severity::Error, Condition::Always),
        };

        assert!(rule.run(&commit).is_some());
    }

    #[test]
    fn test_severity_off() {
        let mut commit: Commit<'_> = Default::default();
        commit.scope = Some(Default::default());

        // If the severity is `Off`, the rule should return `None`
        let rule = ScopeEmptyRule {
            opts: NoOpts(Severity::Off, Condition::Never),
        };

        assert!(rule.run(&commit).is_none());

        let rule = ScopeEmptyRule {
            opts: NoOpts(Severity::Off, Condition::Always),
        };

        assert!(rule.run(&commit).is_none());
    }
}
