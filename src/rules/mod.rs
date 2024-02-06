use config::Config;
use serde::Deserialize;

use crate::parser::Commit;

pub mod scope_empty;
pub mod scope_enum;
pub mod scope_max_length;

pub(crate) trait Rule {
    fn run(&self, commit: &Commit) -> Option<miette::Report>;
}

/// Severity of the rule
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) enum Severity {
    /// Turn off the rule
    #[serde(rename = "off")]
    Off,
    /// Warn about the violation of a rule
    #[serde(rename = "warning")]
    Warning,
    /// Error about the violation of a rule
    #[serde(rename = "error")]
    Error,
}

/// When the rule should be applied
#[derive(Debug, Deserialize)]
pub(crate) enum Condition {
    /// The options should "never" be found (e.g. in a list of disallowed values)
    #[serde(rename = "never")]
    Never,
    /// The options should "always" be found (e.g. in a list of allowed values)
    #[serde(rename = "always")]
    Always,
}

/// Possible target cases for the rule (e.g. subject must start with a capital letter: `TargetCase::Sentence`)
#[derive(Debug, Deserialize)]
enum TargetCase {
    /// Lower case (e.g. `sometext`)
    #[serde(rename = "lower-case")]
    Lower,
    /// Upper case (e.g. `SOMETEXT`)
    #[serde(rename = "upper-case")]
    Upper,
    /// Pascal case (e.g. `SomeText`)
    #[serde(rename = "pascal-case")]
    Pascal,
    /// Camel case (e.g. `someText`)
    #[serde(rename = "camel-case")]
    Camel,
    /// Kebab case (e.g. `some-text`)
    #[serde(rename = "kebab-case")]
    Kebab,
    /// Snake case (e.g. `some_text`)
    #[serde(rename = "snake-case")]
    Snake,
    /// Start case (e.g. `Some Text`)
    #[serde(rename = "start-case")]
    Start,
    /// Sentence case (e.g. `Some text`)
    #[serde(rename = "sentence-case")]
    Sentence,
}

/// Options for all rules without options
#[derive(Debug, Deserialize)]
pub(crate) struct NoOpts(Severity, Condition);
/// Options for all enum rules
type EnumOpts = (Severity, Condition, Vec<String>);
/// Options for all length rules
type LengthOpts = (Severity, usize);
/// Options for all case rules
type CaseOpts = (Severity, Condition, TargetCase);

/// Config all the rules
#[derive(Debug, Deserialize)]
struct RulesDetails {
    #[serde(rename = "scope-empty")]
    scope_empty: NoOpts,
    #[serde(rename = "scope-enum")]
    scope_enum: EnumOpts,
    #[serde(rename = "scope-max-length")]
    scope_max_length: LengthOpts,
    #[serde(rename = "scope-case")]
    scope_case: CaseOpts,
}

/// Config
#[derive(Debug, Deserialize)]
struct RulesConfig {
    rules: RulesDetails,
}

pub struct LintResult {
    errors: Option<Vec<miette::Report>>,
    warnings: Option<Vec<miette::Report>>,
}

impl LintResult {
    pub fn errors(&self) -> Option<&Vec<miette::Report>> {
        self.errors.as_ref()
    }

    pub fn errors_len(&self) -> usize {
        match self.errors() {
            None => return 0,
            Some(errors) => errors.len(),
        }
    }

    pub fn has_errors(&self) -> bool {
        self.errors.is_some() && !self.errors().unwrap().is_empty()
    }

    pub fn warnings(&self) -> Option<&Vec<miette::Report>> {
        self.warnings.as_ref()
    }

    pub fn warnings_len(&self) -> usize {
        match self.warnings() {
            None => return 0,
            Some(warnings) => warnings.len(),
        }
    }

    pub fn has_warnings(&self) -> bool {
        self.warnings.is_some() && !self.warnings().unwrap().is_empty()
    }
}

pub fn run(commit: &Commit) -> LintResult {
    let settings = Config::builder()
        // Source can be `commitlint.config.toml` or `commitlint.config.json``
        .add_source(config::File::with_name("src/commitlint.config"))
        .build()
        .unwrap();

    // Print out our settings
    let config: RulesConfig = settings.try_deserialize::<RulesConfig>().unwrap();
    println!("{:?}", config);

    // create list of rules to iterate over them
    let rules: Vec<Box<dyn Rule>> = vec![
        Box::new(scope_empty::ScopeEmptyRule {
            opts: config.rules.scope_empty,
        }),
        Box::new(scope_enum::ScopeEnumRule {
            opts: config.rules.scope_enum,
        }),
        Box::new(scope_max_length::ScopeMaxLengthRule {
            opts: config.rules.scope_max_length,
        }),
    ];

    // iterate over all rules and run them and return all found errors and warnings
    let mut lint_result = LintResult {
        errors: None,
        warnings: None,
    };
    for rule in rules {
        if let Some(report) = rule.run(&commit) {
            match report.severity() {
                Some(miette::Severity::Error) => {
                    if lint_result.errors.is_none() {
                        lint_result.errors = Some(vec![]);
                    }
                    lint_result.errors.as_mut().unwrap().push(report);
                }
                Some(miette::Severity::Warning) => {
                    if lint_result.warnings.is_none() {
                        lint_result.warnings = Some(vec![]);
                    }
                    lint_result.warnings.as_mut().unwrap().push(report);
                }
                _ => {}
            }
        }
    }

    return lint_result;
}
