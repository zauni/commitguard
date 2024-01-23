mod parser;

use crate::parser::{parse_commit, Commit};
use miette::{miette, LabeledSpan, Report, Result};

fn rule_scope_enum(commit: &Commit) -> Option<Report> {
    let allowed_scopes = vec!["backend", "frontend", "api"];

    if let Some(scope) = commit.scope {
        if !allowed_scopes.contains(&scope.as_str()) {
            return Some(
                miette!(
                    severity = miette::Severity::Error,
                    labels = vec![LabeledSpan::at(
                        scope.start()..scope.end(),
                        "not allowed scope"
                    ),],
                    help = "allowed scopes: backend, frontend, api",
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

fn main() -> Result<()> {
    let commit_message =
        "feat(nice): add cool feature\n\nsome body\n\nsecond body line\n\nsome footer";

    let commit = parse_commit(&commit_message);
    println!("{:#?}", commit);

    if let Some(report) = rule_scope_enum(&commit) {
        return Err(report);
    }

    Ok(())
}
