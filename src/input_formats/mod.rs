use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

pub mod sast;

#[derive(Deserialize, Serialize, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
enum Severity {
    #[default]
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl Display for Severity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Info => write!(f, "Info"),
            Severity::Low => write!(f, "Low"),
            Severity::Medium => write!(f, "Medium"),
            Severity::High => write!(f, "High"),
            Severity::Critical => write!(f, "Critical"),
        }
    }
}

pub trait ReportFormatHandler {
    type ReportFormat;
    fn parse_to_struct(s: &str) -> Self::ReportFormat;
    fn render_to_markdown(doc: &Self::ReportFormat) -> String;
}

use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum Handlers {
    Sast,
}

impl Handlers {
    pub fn get_handler(&self) -> impl ReportFormatHandler {
        match self {
            Handlers::Sast => sast::SastHandler,
        }
    }
}
