use clap::Parser;
use std::io;
use std::io::Read;
use std::process::exit;
use exitfailure::ExitFailure;
use human_panic::{setup_panic};
use pretty_env_logger;
use pretty_env_logger::env_logger::Builder;
use log::{debug, error, info, warn};

mod input_formats;
mod poster;

use crate::input_formats::sast::SastHandler;
use crate::input_formats::Handlers;
use crate::input_formats::ReportFormatHandler;

const EMPTY_FILE_ERROR: i32 = 1;
const BAD_CONFIG: i32 = 2;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 't', long = "type", value_enum)]
    handler: Handlers,
    #[arg(
    short,
    long,
    help = "don't try to actually post the message, just parse the report."
    )]
    dry_run: bool,
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


fn main() -> Result<(), ExitFailure> {
    setup_panic!();
    Builder::from_env("LOG_LEVEL").format_level(true).format_module_path(false).format_timestamp(None).init();
    let args = Args::parse();
    let mut raw_doc = String::new();
    let raw_doc_len = match io::stdin()
        .read_to_string(&mut raw_doc) {
        Ok(size) => size,
        Err(e) => {
            error!("Could not read from stdin: {}", e);
            exit(EMPTY_FILE_ERROR);
        }
    };
    if raw_doc_len == 0 {
        error!("Got an empty file on stdin.");
        exit(EMPTY_FILE_ERROR);
    }
    let comment = render_to_comment(args.handler, &raw_doc);
    println!("{}", comment);
    // then upload as gitlab MR comment if we can find the appropriate envvars
    if !args.dry_run {
        poster::post_to_merge_request(&comment)
            .expect("Could not post. Maybe you are missing some envvars?");
    } else {
        warn!("As --dry-run has been specified, I will stop here.");
    }
    Ok(())
}
