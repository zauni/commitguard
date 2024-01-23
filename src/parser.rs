use pest::{Parser, Span};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "commit.pest"]
struct CommitParser;

#[derive(Debug)]
pub struct Commit<'a> {
    /// The complete header of the commit message including the type, scope and subject
    pub header: Span<'a>,
    /// The body of the commit message
    pub body: Option<Span<'a>>,
    /// The footer of the commit message
    pub footer: Option<Span<'a>>,
    /// The type of the commit message (e.g. feat, fix, chore, ...)
    pub commit_type: Span<'a>,
    /// The scope of the commit message (e.g. backend, frontend, ...)
    pub scope: Option<Span<'a>>,
    /// The subject of the commit message
    pub subject: Span<'a>,
    /// The raw commit message
    pub raw: String,
}

pub fn parse_commit(commit_msg: &str) -> Commit {
    let pairs = CommitParser::parse(Rule::commit, commit_msg).unwrap_or_else(|e| panic!("{}", e));
    println!("{:#?}", pairs);

    let mut commit = Commit {
        header: Option::expect(Span::new(&"", 0, 0), "span"),
        body: None,
        footer: None,
        commit_type: Option::expect(Span::new(&"", 0, 0), "span"),
        scope: None,
        subject: Option::expect(Span::new(&"", 0, 0), "span"),
        raw: String::from(""),
    };

    for pair in pairs {
        match pair.as_rule() {
            Rule::commit => {
                commit.raw = String::from(pair.as_str());

                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::header => {
                            commit.header = inner_pair.as_span();

                            for header_pair in inner_pair.into_inner() {
                                match header_pair.as_rule() {
                                    Rule::commit_type => commit.commit_type = header_pair.as_span(),
                                    Rule::scope => commit.scope = Some(header_pair.as_span()),
                                    Rule::subject => commit.subject = header_pair.as_span(),
                                    _ => {}
                                }
                            }
                        }
                        Rule::body => commit.body = Some(inner_pair.as_span()),
                        Rule::footer => commit.footer = Some(inner_pair.as_span()),
                        Rule::commit_type => commit.commit_type = inner_pair.as_span(),
                        Rule::scope => commit.scope = Some(inner_pair.as_span()),
                        Rule::subject => commit.subject = inner_pair.as_span(),
                        _ => {}
                    }
                }
            }
            _ => {}
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
                name: String::from("subject missing"),
                commit: String::from("feat: add cool feature\n\nsome body\n\nsome footer"),
                want_err: false,
            },
            TestConfig {
                name: String::from("body and footer missing"),
                commit: String::from("feat(nice): add cool feature"),
                want_err: false,
            },
            TestConfig {
                name: String::from("footer missing"),
                commit: String::from("feat(nice): add cool feature\n\nsome body"),
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
                name: String::from("type missing"),
                commit: String::from("add cool feature\n\nsome body\n\nsome footer"),
                want_err: true,
            },
            TestConfig {
                name: String::from("not enough newlines"),
                commit: String::from("feat: add cool feature\nsome body"),
                want_err: true,
            },
        ];

        for test_config in test_configs {
            let result = CommitParser::parse(Rule::commit, &test_config.commit);
            if test_config.want_err {
                assert!(
                    result.is_err(),
                    "{} | commit parse error = {:#?}",
                    test_config.name,
                    result
                );
            } else {
                assert!(
                    result.is_ok(),
                    "{} | commit parse should be successfull",
                    test_config.name
                );
            }
        }
    }
}
