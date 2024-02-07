mod parser;
mod rules;

use std::process::ExitCode;

use miette::GraphicalReportHandler;
use parser::parse_commit;

fn main() -> ExitCode {
    let commit_message =
        "feat(nice): add cool feature\n\nsome body\n\nsecond body line\n\nsome footer";

    let commit = parse_commit(&commit_message);
    println!("{:#?}", commit);

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

    if lint_result.has_errors() {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
