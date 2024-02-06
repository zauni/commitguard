use crate::parser::Commit;

use super::{LengthOpts, Rule, Severity};
use miette::{miette, LabeledSpan, Report};

pub struct ScopeMaxLengthRule {
    pub opts: LengthOpts,
}

impl Rule for ScopeMaxLengthRule {
    fn run(&self, commit: &Commit) -> Option<Report> {
        let severity = &self.opts.0;
        let max_length = &self.opts.1;

        if severity == &Severity::Off {
            return None;
        }

        if let Some(scope) = &commit.scope {
            let is_valid = scope.to_string().len() <= *max_length;
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
                        help = String::from("scope must not be longer than ")
                            + &max_length.to_string()
                            + " characters (current length: "
                            + &scope.to_string().len().to_string()
                            + ")",
                        code = "rule/scope-max-length",
                        url = "https://example.com",
                        "Scope too long",
                    )
                    .with_source_code(commit.raw.clone()),
                );
            }
        }

        None
    }
}
