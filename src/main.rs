use clap::Parser;
use std::io;
use std::io::Read;

mod input_formats;
mod poster;

use crate::input_formats::sast::SastHandler;
use crate::input_formats::Handlers;
use crate::input_formats::ReportFormatHandler;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 't', long = "type", value_enum)]
    handler: Handlers,
    #[arg(short, long)]
    dry_run: bool,
}

fn render_to_comment(handler: Handlers, raw_doc: &str) -> String {
    match handler {
        Handlers::Sast => SastHandler::render_to_markdown(&SastHandler::parse_to_struct(raw_doc)),
    }
}

fn main() {
    let args = Args::parse();
    let mut raw_doc = String::new();
    let raw_doc_len = io::stdin()
        .read_to_string(&mut raw_doc)
        .expect("Could not read from stdin");
    if raw_doc_len == 0 {
        panic!("Read a string of length 0 from stdin!");
    }
    let comment = render_to_comment(args.handler, &raw_doc);
    println!("{}", comment);
    // then upload as gitlab MR comment if we can find the appropriate envvars
    poster::post_to_merge_request(&comment)
        .expect("Could not post. Maybe you are missing some envvars?");
}
