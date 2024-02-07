use crate::parser::Commit;

use super::{Condition, EnumOpts, Rule, Severity};
use miette::{miette, LabeledSpan, Report};

pub struct ScopeEnumRule {
    pub opts: EnumOpts,
}

impl Rule for ScopeEnumRule {
    fn run(&self, commit: &Commit) -> Option<Report> {
        let severity = &self.opts.0;
        let condition = &self.opts.1;
        let scopes = &self.opts.2;

        if severity == &Severity::Off || scopes.is_empty() {
            return None;
        }

        if let Some(scope) = &commit.scope {
            let is_in_scopes = scopes.contains(&scope.to_string());
            let is_valid = match condition {
                Condition::Never => !is_in_scopes,
                Condition::Always => is_in_scopes,
            };
            if !is_valid {
                return Some(
                    miette!(
                        severity = match severity {
                            Severity::Warning => miette::Severity::Warning,
                            Severity::Error => miette::Severity::Error,
                            Severity::Off => miette::Severity::Advice,
                        },
                        labels = vec![LabeledSpan::at(
                            scope.start()..scope.end(),
                            "not allowed scope"
                        ),],
                        help = String::from("scope must")
                            + match condition {
                                Condition::Never => " not",
                                Condition::Always => "",
                            }
                            + " be one of "
                            + &scopes.join(", "),
                        code = "rule/scope-enum",
                        url = "https://example.com",
                        "Scope not allowed",
                    )
                    .with_source_code(commit.raw.clone()),
                );
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::CommitSpan;

    use super::*;

    #[test]
    fn test_empty_scope() {
        let mut commit: Commit<'_> = Default::default();
        commit.scope = None;

        // If the scope is empty this rule behaves the same as it would be Severity::Off
        let rule = ScopeEnumRule {
            opts: EnumOpts(
                Severity::Error,
                Condition::Never,
                vec!["feat".to_string(), "fix".to_string()],
            ),
        };

        assert!(rule.run(&commit).is_none());

        let rule = ScopeEnumRule {
            opts: EnumOpts(
                Severity::Error,
                Condition::Always,
                vec!["feat".to_string(), "fix".to_string()],
            ),
        };

        assert!(rule.run(&commit).is_none());
    }

    #[test]
    fn test_never_condition() {
        let mut commit: Commit<'_> = Default::default();

        // If the condition is `Never` the scope should not be in the list of disallowed scopes
        let rule = ScopeEnumRule {
            opts: EnumOpts(
                Severity::Error,
                Condition::Never,
                vec!["feat".to_string(), "fix".to_string()],
            ),
        };

        // an empty scope is not disallowed, so return None
        commit.scope = Some(Default::default());
        assert!(rule.run(&commit).is_none());

        // the scope "nice" is not disallowed, so return None
        commit.scope = Some(CommitSpan::new("nice", 0, 4));
        assert!(rule.run(&commit).is_none());

        // the scope "feat" is disallowed, so return a Report
        commit.scope = Some(CommitSpan::new("feat", 0, 4));
        assert!(rule.run(&commit).is_some());
    }

    #[test]
    fn test_always_condition() {
        let mut commit: Commit<'_> = Default::default();

        // If the condition is `Always` the scope must be in the list of allowed scopes
        let rule = ScopeEnumRule {
            opts: EnumOpts(
                Severity::Error,
                Condition::Always,
                vec!["feat".to_string(), "fix".to_string()],
            ),
        };

        // an empty scope is not allowed, so return a Report
        commit.scope = Some(Default::default());
        assert!(rule.run(&commit).is_some());

        // the scope "nice" is not allowed, so return a Report
        commit.scope = Some(CommitSpan::new("nice", 0, 4));
        assert!(rule.run(&commit).is_some());

        // the scope "feat" is allowed, so return None
        commit.scope = Some(CommitSpan::new("feat", 0, 4));
        assert!(rule.run(&commit).is_none());
    }

    #[test]
    fn test_severity_off() {
        let mut commit: Commit<'_> = Default::default();

        // If the severity is `Off`, the rule should return `None`
        let rule = ScopeEnumRule {
            opts: EnumOpts(
                Severity::Off,
                Condition::Always,
                vec!["feat".to_string(), "fix".to_string()],
            ),
        };

        commit.scope = Some(Default::default());
        assert!(rule.run(&commit).is_none());
        commit.scope = Some(CommitSpan::new("nice", 0, 4));
        assert!(rule.run(&commit).is_none());
        commit.scope = Some(CommitSpan::new("feat", 0, 4));
        assert!(rule.run(&commit).is_none());
    }

    #[test]
    fn test_empty_list() {
        let mut commit: Commit<'_> = Default::default();

        let rule = ScopeEnumRule {
            opts: EnumOpts(Severity::Error, Condition::Always, vec![]),
        };

        commit.scope = Some(Default::default());
        assert!(rule.run(&commit).is_none());
        commit.scope = Some(CommitSpan::new("nice", 0, 4));
        assert!(rule.run(&commit).is_none());
        commit.scope = Some(CommitSpan::new("feat", 0, 4));
        assert!(rule.run(&commit).is_none());
    }
}
