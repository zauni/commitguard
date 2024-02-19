mod parser;
mod rules;

use std::{env::current_dir, io::{stdin, Read}, path::PathBuf, process::ExitCode};

use clap::Parser;
use miette::GraphicalReportHandler;
use parser::parse_commit;

/// Commit lint
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the config file
    #[arg(short, long, default_value = "commitlint.config")]
    config_path: PathBuf,

    /// Current working directory
    #[arg(long, default_value = current_dir().unwrap_or_else(|_e| PathBuf::from("/")).into_os_string())]
    cwd: PathBuf,
}

fn main() -> ExitCode {
    // let commit_message =
    //     "feat(nice): add cool feature\n\nsome body\n\nsecond body line\n\nsome footer";

    // let commit = parse_commit(&commit_message);
    // println!("{:#?}", commit);

    // read commit from stdin
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap_or(0);
    let commit = parse_commit(&buffer);

    let lint_result = rules::run(&commit);
    let report_handler = GraphicalReportHandler::new();

    if lint_result.has_warnings() {
        let mut out = String::new();
        lint_result.warnings().unwrap().iter().for_each(|report| {
            out.push('\n');
            let _ = report_handler.render_report(&mut out, report.as_ref());
        });

        println!("{}", out);
    }

    if lint_result.has_errors() {
        let mut out = String::new();
        lint_result.errors().unwrap().iter().for_each(|report| {
            out.push('\n');
            let _ = report_handler.render_report(&mut out, report.as_ref());
        });

        println!("{}", out);
    }

    println!(
        "There are {} warnings and {} errors",
        lint_result.warnings_len(),
        lint_result.errors_len()
    );

    let args = Cli::parse();
    println!("{:?}", args);

    if lint_result.has_errors() {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
