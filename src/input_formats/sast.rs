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
    #[serde(default)]
    dependency_files: Vec<()>,
    #[serde(default)]
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
            name: "unspecified".to_string()
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct VulnerabilityLocation {
    file: String,
    start_line: u32,
    end_line: Option<u32>,
}

impl Default for VulnerabilityLocation {
    fn default() -> Self {
        Self {
            file: "unspecified".to_string(),
            start_line: 1,
            end_line: None
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct VulnerabilityIdentifier {
    #[serde(rename = "type")]
    vulnerability_type: String,
    name: String,
    value: String,
    url: String,
}
impl Default for VulnerabilityIdentifier {
    fn default() -> Self {
        Self {
            vulnerability_type: "unspecified".to_string(),
            name: "unspecified".to_string(),
            value: "unspecified".to_string(),
            url: "unspecified".to_string()
        }
    }
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
        Self {name:"unspecified".to_string()}
    }
}
