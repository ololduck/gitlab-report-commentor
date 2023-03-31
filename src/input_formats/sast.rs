use askama::Template;
use chrono::NaiveDateTime;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use super::{ReportFormatHandler, Severity};

pub struct SastHandler;

const VULNERABILITY_REPORT_TEMPLATE: &str =
    include_str!("../../templates/sast/vulnerability_report.md.hbs");

impl ReportFormatHandler for SastHandler {
    type ReportFormat = SastReport;
    fn parse_to_struct(s: &str) -> Self::ReportFormat {
        serde_json::from_str(s).expect("Could not parse given string as proper SAST report")
    }

    fn render_to_markdown(doc: &Self::ReportFormat) -> String {
        let mut hbs = Handlebars::new();
        hbs.register_template_string(
            "VULNERABILITY_REPORT_TEMPLATE",
            VULNERABILITY_REPORT_TEMPLATE,
        )
        .unwrap();
        hbs.render("VULNERABILITY_REPORT_TEMPLATE", &doc).unwrap()
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SastReport {
    version: String,
    vulnerabilities: Vec<Vulnerability>,
    #[serde(default)]
    dependency_files: Vec<()>,
    scan: Option<Scan>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Vulnerability {
    id: Option<String>,
    category: String,
    #[serde(default)]
    name: String,
    message: String,
    description: String,
    #[serde(default)]
    cve: String,
    #[serde(default)]
    severity: Severity,
    #[serde(default)]
    scanner: VulnerabilityScanner,
    #[serde(default)]
    location: VulnerabilityLocation,
    #[serde(default)]
    identifiers: Vec<VulnerabilityIdentifier>,
}

#[derive(Deserialize, Serialize, Debug)]
struct VulnerabilityScanner {
    id: String,
    name: String,
}
impl Default for VulnerabilityScanner {
    fn default() -> Self {
        Self {
            id: "unspecified".to_string(),
            name: "unspecified".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct VulnerabilityLocation {
    file: Option<String>,
    start_line: Option<u32>,
    end_line: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct VulnerabilityIdentifier {
    #[serde(rename = "type")]
    vulnerability_type: String,
    name: String,
    value: String,
    url: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Scan {
    #[serde(default)]
    analyzer: Software,
    #[serde(default)]
    scanner: Software,
    #[serde(rename = "type")]
    scan_type: String,
    #[serde(default)]
    start_time: NaiveDateTime,
    #[serde(default)]
    end_time: NaiveDateTime,
    status: String,
}
impl Default for Scan {
    fn default() -> Self {
        Self {
            scan_type: "unspecified".to_string(),
            status: "unspecified".to_string(),
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Software {
    id: String,
    name: String,
    url: String,
    #[serde(default)]
    vendor: SoftwareVendor,
    version: String,
}
impl Default for Software {
    fn default() -> Self {
        Self {
            id: "unspecified".to_string(),
            name: "unspecified".to_string(),
            url: "unspecified".to_string(),
            version: "unspecified".to_string(),
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct SoftwareVendor {
    name: String,
}
impl Default for SoftwareVendor {
    fn default() -> Self {
        Self {
            name: "unspecified".to_string(),
        }
    }
}
