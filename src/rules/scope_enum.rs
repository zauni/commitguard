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

        if severity == &Severity::Off {
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
