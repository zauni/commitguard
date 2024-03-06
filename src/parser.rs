use std::fmt;

use serde::Serialize;

use pest::{Parser, Span};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "commit.pest"]
struct CommitParser;

/// A span of a part of the commit message
#[derive(Debug, Serialize, Default)]
pub struct CommitSpan<'a> {
    input: &'a str,
    start: usize,
    end: usize,
}

impl<'a> CommitSpan<'a> {
    pub fn new(input: &'a str, start: usize, end: usize) -> Self {
        CommitSpan { input, start, end }
    }

    fn from(span: Span<'a>) -> Self {
        CommitSpan {
            input: span.as_str(),
            start: span.start(),
            end: span.end(),
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

impl fmt::Display for CommitSpan<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.input)
    }
}

#[derive(Debug, Serialize)]
pub struct Commit<'a> {
    /// The complete header of the commit message including the type, scope and subject
    pub header: CommitSpan<'a>,
    /// The body of the commit message
    pub body: Option<CommitSpan<'a>>,
    /// The footer of the commit message
    pub footer: Option<CommitSpan<'a>>,
    /// The type of the commit message (e.g. feat, fix, chore, ...)
    pub commit_type: CommitSpan<'a>,
    /// The scope of the commit message (e.g. backend, frontend, ...)
    pub scope: Option<CommitSpan<'a>>,
    /// The subject of the commit message
    pub subject: CommitSpan<'a>,
    /// The raw commit message
    pub raw: String,
}

impl Commit<'_> {
    pub fn new() -> Self {
        Commit {
            header: CommitSpan::default(),
            body: None,
            footer: None,
            commit_type: CommitSpan::default(),
            scope: None,
            subject: CommitSpan::default(),
            raw: String::from(""),
        }
    }
}

impl Default for Commit<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse_commit(commit_msg: &str) -> Commit {
    let pairs = CommitParser::parse(Rule::commit, commit_msg).unwrap_or_else(|e| panic!("{}", e));
    println!("{:#?}", pairs);

    let mut commit = Commit {
        header: CommitSpan::new("", 0, 0),
        body: None,
        footer: None,
        commit_type: CommitSpan::new("", 0, 0),
        scope: None,
        subject: CommitSpan::new("", 0, 0),
        raw: String::from(""),
    };

    for pair in pairs {
        if let Rule::commit = pair.as_rule() {
            commit.raw = pair.as_str().to_string();

            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::header => {
                        commit.header = CommitSpan::from(inner_pair.as_span());

                        for header_pair in inner_pair.into_inner() {
                            match header_pair.as_rule() {
                                Rule::commit_type => {
                                    commit.commit_type = CommitSpan::from(header_pair.as_span())
                                }
                                Rule::scope => {
                                    commit.scope = Some(CommitSpan::from(header_pair.as_span()))
                                }
                                Rule::subject => {
                                    commit.subject = CommitSpan::from(header_pair.as_span())
                                }
                                _ => {}
                            }
                        }
                    }
                    Rule::body => commit.body = Some(CommitSpan::from(inner_pair.as_span())),
                    Rule::footer => commit.footer = Some(CommitSpan::from(inner_pair.as_span())),
                    Rule::commit_type => {
                        commit.commit_type = CommitSpan::from(inner_pair.as_span())
                    }
                    Rule::scope => commit.scope = Some(CommitSpan::from(inner_pair.as_span())),
                    Rule::subject => commit.subject = CommitSpan::from(inner_pair.as_span()),
                    _ => {}
                }
            }
        }
    }

    commit
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig {
        /// `name` is used for test identification
        name: String,
        /// `commit` is the commit message to be parsed
        commit: String,
        /// `want_err` is used to indicate if the commit should be parsed successfully or not
        want_err: bool,
    }

    #[test]
    fn commit_parse_tests() {
        let test_configs = vec![
            TestConfig {
                name: String::from("complex"),
                commit: String::from("feat(nice): add cool feature\n\nsome body\n\nsome footer"),
                want_err: false,
            },
            TestConfig {
                name: String::from("scope missing"),
                commit: String::from("feat: add cool feature\n\nsome body\n\nsome footer"),
                want_err: false,
            },
            TestConfig {
                name: String::from("body and footer missing"),
                commit: String::from("feat(nice): add cool feature"),
                want_err: false,
            },
            TestConfig {
                name: String::from(
                    "body and footer missing with newline at the end (stdio input adds a newline)",
                ),
                commit: String::from("feat(nice): add cool feature\n"),
                want_err: false,
            },
            TestConfig {
                name: String::from("subject with whitespace at the end"),
                commit: String::from("feat(nice): add cool feature \t "),
                want_err: false,
            },
            TestConfig {
                name: String::from("footer missing"),
                commit: String::from("feat(nice): add cool feature\n\nsome body"),
                want_err: false,
            },
            TestConfig {
                name: String::from("multiple body lines"),
                commit: String::from(
                    "feat(nice): add cool feature\n\nsome body\nnext body line\n\nthe real footer",
                ),
                want_err: false,
            },
            TestConfig {
                name: String::from("breaking change after type"),
                commit: String::from("feat!: add cool feature\n\nsome body"),
                want_err: false,
            },
            TestConfig {
                name: String::from("breaking change after scope"),
                commit: String::from("feat(nice)!: add cool feature\n\nsome body"),
                want_err: false,
            },
            TestConfig {
                name: String::from("only one newline after header"),
                commit: String::from("feat(nice): add cool feature\nsome body"),
                want_err: true,
            },
            TestConfig {
                name: String::from("type missing"),
                commit: String::from("add cool feature\n\nsome body\n\nsome footer"),
                want_err: true,
            },
            TestConfig {
                name: String::from("not enough newlines"),
                commit: String::from("feat: add cool feature\nsome body"),
                want_err: true,
            },
            TestConfig {
                name: String::from("random text"),
                commit: String::from("Added a cool new feature"),
                want_err: true,
            },
            TestConfig {
                name: String::from("no text"),
                commit: String::new(),
                want_err: true,
            },
        ];

        for test_config in test_configs {
            let parse_result = CommitParser::parse(Rule::commit, &test_config.commit);

            if test_config.want_err {
                assert!(
                    parse_result.is_err(),
                    "{} | commit parse error = {:#?}",
                    test_config.name,
                    parse_result
                );
            } else {
                let result = parse_commit(&test_config.commit);
                assert!(
                    parse_result.is_ok(),
                    "{} | commit parse should be successfull",
                    test_config.name
                );
                insta::assert_yaml_snapshot!(test_config.name, result);
            }
        }
    }
}
