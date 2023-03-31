use askama::Template;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{ReportFormatHandler, Severity};

pub struct SastHandler;

#[derive(Template)]
#[template(path = "sast/vulnerability_report.md.j2")]
struct VulnerabilityReportTemplate<'a> {
    vulnerabilities: &'a Vec<Vulnerability>,
    scan: &'a Scan,
}

impl ReportFormatHandler for SastHandler {
    type ReportFormat = SastReport;
    fn parse_to_struct(s: &str) -> Self::ReportFormat {
        serde_json::from_str(s).expect("Could not parse given string as proper SAST report")
    }

    fn render_to_markdown(doc: &Self::ReportFormat) -> String {
        let report = VulnerabilityReportTemplate {
            vulnerabilities: &doc.vulnerabilities,
            scan: &doc.scan,
        };
        report.render().unwrap()
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SastReport {
    version: String,
    vulnerabilities: Vec<Vulnerability>,
    dependency_files: Vec<()>,
    scan: Scan,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Vulnerability {
    id: String,
    category: String,
    #[serde(default)]
    name: String,
    message: String,
    description: String,
    cve: String,
    severity: Severity,
    scanner: VulnerabilityScanner,
    location: VulnerabilityLocation,
    identifiers: Vec<VulnerabilityIdentifier>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct VulnerabilityScanner {
    id: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct VulnerabilityLocation {
    file: String,
    start_line: u32,
    end_line: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct VulnerabilityIdentifier {
    #[serde(rename = "type")]
    vulnerability_type: String,
    name: String,
    value: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Scan {
    analyzer: Software,
    scanner: Software,
    #[serde(rename = "type")]
    scan_type: String,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    status: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Software {
    id: String,
    name: String,
    url: String,
    vendor: SoftwareVendor,
    version: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct SoftwareVendor {
    name: String,
}
