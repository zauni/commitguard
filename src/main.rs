use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "commit.pest"]
struct CommitParser;

#[derive(Debug)]
struct Commit {
    /// The complete header of the commit message including the type, scope and subject
    header: String,
    /// The body of the commit message
    body: Option<String>,
    /// The footer of the commit message
    footer: Option<String>,
    /// The type of the commit message (e.g. feat, fix, chore, ...)
    commit_type: String,
    /// The scope of the commit message (e.g. backend, frontend, ...)
    scope: Option<String>,
    /// The subject of the commit message
    subject: String,
    /// The raw commit message
    raw: String,
}

fn parse_commit(pairs: pest::iterators::Pairs<Rule>) -> Commit {
    let mut commit = Commit {
        header: String::new(),
        body: None,
        footer: None,
        commit_type: String::new(),
        scope: None,
        subject: String::new(),
        raw: String::new(),
    };

    for pair in pairs {
        match pair.as_rule() {
            Rule::commit => {
                commit.raw = pair.as_str().to_string();

                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::header => {
                            commit.header = inner_pair.as_str().to_string();

                            for header_pair in inner_pair.into_inner() {
                                match header_pair.as_rule() {
                                    Rule::commit_type => {
                                        commit.commit_type = header_pair.as_str().to_string()
                                    }
                                    Rule::scope => {
                                        commit.scope = Some(header_pair.as_str().to_string())
                                    }
                                    Rule::subject => {
                                        commit.subject = header_pair.as_str().to_string()
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Rule::body => commit.body = Some(inner_pair.as_str().to_string()),
                        Rule::footer => commit.footer = Some(inner_pair.as_str().to_string()),
                        Rule::commit_type => commit.commit_type = inner_pair.as_str().to_string(),
                        Rule::scope => commit.scope = Some(inner_pair.as_str().to_string()),
                        Rule::subject => commit.subject = inner_pair.as_str().to_string(),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    commit
}

fn main() {
    let commit_message =
        "feat(nice): add cool feature\n\nsome body\n\nsecond body line\n\nsome footer";

    let parsed =
        CommitParser::parse(Rule::commit, &commit_message).unwrap_or_else(|e| panic!("{}", e));
    println!("{:#?}", parsed);

    let commit = parse_commit(parsed);
    println!("{:#?}", commit);
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
