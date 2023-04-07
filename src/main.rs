use clap::{Parser, ValueEnum};
use exitfailure::ExitFailure;
use human_panic::setup_panic;
use log::{debug, error, info, warn};
use pretty_env_logger::env_logger::Builder;
use std::io;
use std::io::Read;
use std::process::exit;

mod input_formats;
mod poster;

use crate::input_formats::sast::SastHandler;
use crate::input_formats::Handlers;
use crate::input_formats::ReportFormatHandler;
use crate::poster::DiffLineComment;

const EXIT_CODE_EMPTY_FILE_ERROR: i32 = 1;
const EXIT_CODE_BAD_CONFIG: i32 = 2;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 't', long = "type", value_enum)]
    report_type: Handlers,
    #[arg(short, long, value_enum, default_value_t=NotifyMode::MRComment)]
    mode: NotifyMode,
    #[arg(
        short,
        long,
        help = "Don't try to actually post the message, just parse the report."
    )]
    dry_run: bool,
}
#[derive(ValueEnum, Clone)]
enum NotifyMode {
    MRComment,
    DiffComment,
}

/// Takes a given handler, matches it with i
fn render_to_comment(handler: Handlers, raw_doc: &str) -> String {
    let s = match handler {
        Handlers::Sast => SastHandler::render_to_markdown(&SastHandler::parse_to_struct(raw_doc)),
    };
    info!("Successfully parsed input");
    debug!("Rendered template:\n{}", &s);
    s
}

fn render_to_diffs(handler: Handlers, raw_doc: &str) -> Vec<DiffLineComment> {
    let v = match handler {
        Handlers::Sast => {
            SastHandler::render_to_diff_line_comments(&SastHandler::parse_to_struct(raw_doc))
        }
    };
    info!("Successfully parsed input");
    v
}

fn main() -> Result<(), ExitFailure> {
    setup_panic!();
    Builder::from_env("LOG_LEVEL")
        .format_level(true)
        .format_module_path(false)
        .format_timestamp(None)
        .init();
    let args = Args::parse();
    let mut raw_doc = String::new();
    let raw_doc_len = match io::stdin().read_to_string(&mut raw_doc) {
        Ok(size) => size,
        Err(e) => {
            error!("Could not read from stdin: {}", e);
            exit(EXIT_CODE_EMPTY_FILE_ERROR);
        }
    };
    if raw_doc_len == 0 {
        error!("Got an empty file on stdin.");
        exit(EXIT_CODE_EMPTY_FILE_ERROR);
    }
    match args.mode {
        NotifyMode::MRComment => {
            let comment = render_to_comment(args.report_type, &raw_doc);
            // then upload as gitlab MR comment if we can find the appropriate envvars
            if !args.dry_run {
                poster::post_to_merge_request(&comment)
                    .expect("Could not post. Maybe you are missing some envvars?");
            } else {
                warn!("As --dry-run has been specified, I will stop here. Here's what would be posted: {}", comment);
            }
        }
        NotifyMode::DiffComment => {
            let diffs = render_to_diffs(args.report_type, &raw_doc);
            if !args.dry_run {
                poster::post_to_diff(&diffs)?;
            } else {
                warn!("As --dry-run has been specified, I will stop here.");
            }
        }
    }
    Ok(())
}
